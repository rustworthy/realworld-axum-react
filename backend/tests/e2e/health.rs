use axum::http::HeaderValue;
use reqwest::{StatusCode, header};
use serde_json::Value;

use crate::utils::TestContext;

async fn healthz_endpoint(ctx: TestContext) {
    // we are calling a healthcheck endpoints (just like the kamal proxy
    // or load balancer periodically does)
    let healthz = format!("{}/healthz", ctx.backend_url);
    let response = ctx.http_client.get(healthz).send().await.unwrap();

    // the status code in 200 and ...
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        HeaderValue::from_str("application/json").unwrap(),
    );

    // ... there is also some payload (we are not inspecting the payload
    // thoroughly since this might change)
    let body = response.bytes().await.unwrap();
    let payload: Value = serde_json::from_slice(&body).unwrap();
    assert!(payload.as_object().unwrap().get("version").is_some());

    // we are also send a test email as part of the health check (to make sure
    // the mailer is configured with the valid API key), so let's verify here
    // that we are actually sending those requests
    let sent_emails = ctx.mailer_server.received_requests().await.unwrap();
    assert_eq!(sent_emails.len(), 1);
    let body: serde_json::Value = sent_emails.last().unwrap().body_json().unwrap();
    assert_eq!(
        body.as_object().unwrap().get("subject").unwrap(),
        "healthcheck endpoint subject"
    )
}

mod tests {
    crate::async_test!(healthz_endpoint);
}
