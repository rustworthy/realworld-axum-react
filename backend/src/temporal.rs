use anyhow::Context as _;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{sync::Arc, time::Duration};
use temporal_client::{Client, RetryClient, WorkflowService, tonic};
use temporal_common::{
    protos::{
        coresdk::AsJsonPayloadExt,
        temporal::api::{
            common::v1::{Payload, WorkflowType},
            enums::v1::{ScheduleOverlapPolicy, TaskQueueKind},
            schedule::v1::{
                Schedule, ScheduleAction, SchedulePolicies, ScheduleSpec, schedule_action::Action,
            },
            taskqueue::v1::TaskQueue,
            workflow::v1::NewWorkflowExecutionInfo,
            workflowservice::v1::{CreateScheduleRequest, CreateScheduleResponse},
        },
    },
    worker::{WorkerTaskTypes, WorkerVersioningStrategy},
};
use temporal_sdk::{ActContext, ActivityOptions, WfContext, Worker, sdk_client_options};
use temporal_sdk_core::{CoreRuntime, RuntimeOptionsBuilder, WorkerConfigBuilder, init_worker};
use url::Url;

pub(crate) type TemporalClient = RetryClient<Client>;

pub(crate) async fn init_client(url: Url) -> anyhow::Result<TemporalClient> {
    let client_options = sdk_client_options(url)
        .build()?
        .connect("default", None)
        .await?;
    Ok(client_options)
}

pub(crate) async fn init_runtime() -> anyhow::Result<CoreRuntime> {
    let rt_opts = RuntimeOptionsBuilder::default().build()?;
    let rt = CoreRuntime::new_assume_tokio(rt_opts)?;
    Ok(rt)
}

pub(crate) async fn create_maintenance_schedule(
    client: &mut TemporalClient,
) -> anyhow::Result<CreateScheduleResponse> {
    let res = client
        .create_schedule(tonic::Request::new(CreateScheduleRequest {
            request_id: "maintenance_create_schedule_request_id_004".into(),
            namespace: "default".into(),
            schedule_id: "maintenance".into(),
            schedule: Some(Schedule {
                spec: Some(ScheduleSpec {
                    cron_string: vec!["@every 20s".into()],
                    ..Default::default()
                }),
                policies: Some(SchedulePolicies {
                    overlap_policy: ScheduleOverlapPolicy::Skip as i32,
                    ..Default::default()
                }),
                action: Some(ScheduleAction {
                    action: Some(Action::StartWorkflow(NewWorkflowExecutionInfo {
                        //workflow_id: "maintenance_workflow".into(),
                        workflow_type: Some(WorkflowType {
                            name: "schedule".into(),
                        }),
                        task_queue: Some(TaskQueue {
                            name: "schedule".into(),
                            //normal_name: "schedule_task_queue".into(),
                            kind: TaskQueueKind::Unspecified as i32,
                            ..Default::default()
                        }),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .await?;
    Ok(res.into_inner())
}

pub(crate) async fn create_maintenance_worker(
    rt: &CoreRuntime,
    db_url: &str,
    client: TemporalClient,
) -> anyhow::Result<Worker> {
    let config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_types(WorkerTaskTypes::all())
        .task_queue("schedule")
        .versioning_strategy(WorkerVersioningStrategy::default())
        .client_identity_override(Some("worker_001".into()))
        .build()?;
    let core_worker = init_worker(rt, config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "schedule");
    let postgres_pool = PgPoolOptions::new()
        .connect(db_url)
        .await
        .context("Failed to connect to database")?;
    worker.register_wf("schedule", move |ctx: WfContext| async move {
        println!(
            "[WORKER][WORKFLOW] task_queue={} args={:?}",
            ctx.task_queue(),
            ctx.get_args(),
        );
        let _res = ctx
            .activity(ActivityOptions {
                activity_type: "confirmation_tokens_clean_up".into(),
                start_to_close_timeout: Some(Duration::from_secs(5)),
                input: Empty.as_json_payload().expect("valid json"),
                ..Default::default()
            })
            .await;
        dbg!(_res);
        Ok(temporal_sdk::WfExitValue::Normal(1003))
    });
    worker.register_activity(
        "confirmation_tokens_clean_up",
        |ctx: ActContext, _input: Empty| async move {
            let pool: &PgPool = ctx.app_data().expect("pool");
            let naffected =
                sqlx::query!("DELETE FROM confirmation_tokens WHERE expires_at >= NOW()")
                    .execute(pool)
                    .await?
                    .rows_affected();
            println!("[WORKER][ACTIVITY] naffected={}", naffected);
            Ok(naffected)
        },
    );
    worker.insert_app_data(postgres_pool);

    Ok(worker)
}

#[derive(Serialize, Deserialize)]
struct Empty;
