use axum::http::HeaderValue;
use reqwest::{StatusCode, header};
use serde_json::Value;

use crate::utils::TestContext;

async fn healthz_endpoint(ctx: TestContext) {
    // we are calling a healthcheck endpoints (just like the kamal proxy
    // or load balancer periodically does)
    let healthz = format!("{}/healthz", ctx.url);
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
}

mod tests {
    crate::async_test!(healthz_endpoint);
}
