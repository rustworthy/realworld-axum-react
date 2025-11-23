use anyhow::Context as _;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{sync::Arc, time::Duration};
use temporal_client::{
    Client, RetryClient, WorkflowService,
    tonic::{self, Status},
};
use temporal_common::{
    protos::{
        coresdk::{AsJsonPayloadExt, FromJsonPayloadExt},
        temporal::api::{
            common::v1::WorkflowType,
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

const SCHEDULE_ID: &str = "scheduled_maintenance_id_001";

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
) -> Result<Option<CreateScheduleResponse>, Status> {
    let response = client
        .create_schedule(tonic::Request::new(CreateScheduleRequest {
            schedule_id: SCHEDULE_ID.into(),
            request_id: format!("{}_create_request_dedup", SCHEDULE_ID),
            namespace: "default".into(),
            schedule: Some(Schedule {
                spec: Some(ScheduleSpec {
                    cron_string: vec!["@every 24h".into()],
                    ..Default::default()
                }),
                policies: Some(SchedulePolicies {
                    overlap_policy: ScheduleOverlapPolicy::Skip as i32,
                    ..Default::default()
                }),
                action: Some(ScheduleAction {
                    action: Some(Action::StartWorkflow(NewWorkflowExecutionInfo {
                        workflow_id: "scheduled_maintenance_workflow".into(),
                        workflow_type: Some(WorkflowType {
                            name: "scheduled_maintenance".into(),
                        }),
                        task_queue: Some(TaskQueue {
                            name: "scheduled_maintenance".into(),
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
        .await
        .context("failed to create schedule");

    // we cannot solely rely on the dedup request ID and need to check
    // if the request failed due to a conflict, which we consider to be ok
    match response {
        Ok(res) => Ok(Some(res.into_inner())),
        Err(e) => {
            let grpc_status = Status::from_error(e.into_boxed_dyn_error());
            if grpc_status.code() == tonic::Code::AlreadyExists {
                info!(schedule_id = SCHEDULE_ID, "schedule already exists");
                Ok(None)
            } else {
                Err(grpc_status)
            }
        }
    }
}

pub(crate) async fn create_maintenance_worker(
    rt: &CoreRuntime,
    db_url: &str,
    client: TemporalClient,
) -> anyhow::Result<Worker> {
    let config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_types(WorkerTaskTypes::all())
        .task_queue("scheduled_maintenance")
        .versioning_strategy(WorkerVersioningStrategy::default())
        .client_identity_override(Some("scheduled_maintenance_worker_001".into()))
        .build()?;
    let core_worker = init_worker(rt, config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "scheduled_maintenance");
    let postgres_pool = PgPoolOptions::new()
        .connect(db_url)
        .await
        .context("Failed to connect to database")?;
    worker.register_wf("scheduled_maintenance", move |ctx: WfContext| async move {
        info!(task_queue = %ctx.task_queue(), "staring workflow execution");
        let payload = ctx
            .activity(ActivityOptions {
                activity_type: "confirmation_tokens_clean_up".into(),
                start_to_close_timeout: Some(Duration::from_secs(5)),
                input: Empty.as_json_payload().expect("valid json"),
                ..Default::default()
            })
            .await
            .success_payload_or_error()?
            .ok_or(anyhow::anyhow!(
                "Expected payload from 'confirmation_tokens_clean_up' activity"
            ))?;
        let result = CleanUpResult::from_json_payload(&payload)
            .context("failed to deserialize activity result")?;
        Ok(temporal_sdk::WfExitValue::Normal(result))
    });
    worker.register_activity(
        "confirmation_tokens_clean_up",
        |ctx: ActContext, _input: Empty| async move {
            let pool: &PgPool = ctx.app_data().expect("PostgrSQL connection pool");
            let naffected =
                sqlx::query!("DELETE FROM confirmation_tokens WHERE expires_at >= NOW()")
                    .execute(pool)
                    .await?
                    .rows_affected();
            ctx.record_heartbeat(vec![CleanUpResult { naffected }.as_json_payload().unwrap()]);
            Ok(CleanUpResult { naffected })
        },
    );
    worker.insert_app_data(postgres_pool);

    Ok(worker)
}

#[derive(Serialize, Deserialize)]
struct Empty;

#[derive(Debug, Serialize, Deserialize)]
struct CleanUpResult {
    naffected: u64,
}
