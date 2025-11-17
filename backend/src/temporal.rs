use temporal_client::{Client, RetryClient, WorkflowService, tonic};
use temporal_common::protos::temporal::api::{
    common::v1::WorkflowType,
    enums::v1::TaskQueueKind,
    schedule::v1::{Schedule, ScheduleAction, ScheduleSpec, schedule_action::Action},
    taskqueue::v1::TaskQueue,
    workflow::v1::NewWorkflowExecutionInfo,
    workflowservice::v1::{CreateScheduleRequest, CreateScheduleResponse},
};
use temporal_sdk::sdk_client_options;
use url::Url;

pub(crate) async fn init_client(url: Url) -> anyhow::Result<RetryClient<Client>> {
    let client_options = sdk_client_options(url)
        .build()?
        .connect("default", None)
        .await?;
    Ok(client_options)
}

pub(crate) async fn create_maintenance_schedule(
    client: &mut RetryClient<Client>,
) -> anyhow::Result<CreateScheduleResponse> {
    let res = client
        .create_schedule(tonic::Request::new(CreateScheduleRequest {
            request_id: "maintenance_create_schedule_request_id_002".into(),
            namespace: "default".into(),
            schedule_id: "maintenance".into(),
            schedule: Some(Schedule {
                spec: Some(ScheduleSpec {
                    cron_string: vec!["@every 20s".into()],
                    ..Default::default()
                }),
                action: Some(ScheduleAction {
                    action: Some(Action::StartWorkflow(NewWorkflowExecutionInfo {
                        workflow_id: "maintenance_workflow".into(),
                        workflow_type: Some(WorkflowType {
                            name: "schedule_workflow_type".into(),
                        }),
                        task_queue: Some(TaskQueue {
                            name: "schedule_task_queue".into(),
                            normal_name: "schedule_task_queue".into(),
                            kind: TaskQueueKind::Unspecified as i32,
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
