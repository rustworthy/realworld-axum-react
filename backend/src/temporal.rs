use temporal_client::{Client, RetryClient, WorkflowService, tonic};
use temporal_common::protos::temporal::api::workflowservice::v1::{
    CreateScheduleRequest, CreateScheduleResponse,
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
            request_id: "maintenance_request_id".into(),
            namespace: "default".into(),
            schedule_id: "maintenance".into(),
            ..Default::default()
        }))
        .await?;
    Ok(res.into_inner())
}
