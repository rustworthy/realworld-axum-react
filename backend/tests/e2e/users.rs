use reqwest::{StatusCode, header};

use crate::utils::TestContext;

// This token has been signed with using a secret in our `.env.example`, while
// for each of our tests we are launching a dedicated rocker application with a
// dedicated random secret key, and so we expect the back-end to reject us
const TEST_JWT_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyNWY3NTMzNy1hNWUzLTQ0YjEtOTdkNy02NjUzY2EyM2U5ZWUiLCJpYXQiOjE3NTEzMTE5NzksImV4cCI6MTc1MTkxNjc3OX0.QJXG34zRbMLin8JUr-BBbwOSQWwaJ9T2VGRDAbLTJ88";

// --------------------------- POST /api/users ---------------------------------
async fn create_user_empty_payload(ctx: TestContext) {
    let url = format!("{}/api/users", ctx.url);
    let response = ctx.http_client.post(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.bytes().await.unwrap().len() > 0);
}

// ------------------------- POST /api/users/login -----------------------------
async fn login_empty_payload(ctx: TestContext) {
    let url = format!("{}/api/users/login", ctx.url);
    let response = ctx.http_client.post(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.bytes().await.unwrap().len() > 0);
}

// ---------------------------- GET /api/user ----------------------------------
async fn get_current_user_no_token(ctx: TestContext) {
    let url = format!("{}/api/user", ctx.url);
    let response = ctx.http_client.get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get(header::WWW_AUTHENTICATE).unwrap(),
        // reminder: Realworld spec wants "Token" here and we are supporting
        // both formats, but encouraging to use "Bearer"
        "Bearer"
    );
    assert!(response.bytes().await.unwrap().is_empty());
}

async fn get_current_user_invalid_token(ctx: TestContext) {
    let url = format!("{}/api/user", ctx.url);
    let response = ctx
        .http_client
        .get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", TEST_JWT_TOKEN))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get(header::WWW_AUTHENTICATE).unwrap(),
        "Bearer"
    );
    assert!(response.bytes().await.unwrap().is_empty());
}

// --------------------------- PUT /api/user -----------------------------------
async fn update_user_unauthenticated(ctx: TestContext) {
    let url = format!("{}/api/user", ctx.url);
    let response = ctx.http_client.put(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get(header::WWW_AUTHENTICATE).unwrap(),
        "Bearer"
    );
    assert!(response.bytes().await.unwrap().is_empty());
}

mod tests {
    crate::async_test!(create_user_empty_payload);
    crate::async_test!(login_empty_payload);
    crate::async_test!(get_current_user_no_token);
    crate::async_test!(get_current_user_invalid_token);
    crate::async_test!(update_user_unauthenticated);
}
