use reqwest::StatusCode;
use serde_json::json;

use crate::utils::TestContext;

// This token has been signed with using a secret in our `.env.example`, while
// for each of our tests we are launching a dedicated rocker application with a
// dedicated random secret key, and so we expect the back-end to reject us
const _TEST_JWT_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyNWY3NTMzNy1hNWUzLTQ0YjEtOTdkNy02NjUzY2EyM2U5ZWUiLCJpYXQiOjE3NTEzMTE5NzksImV4cCI6MTc1MTkxNjc3OX0.QJXG34zRbMLin8JUr-BBbwOSQWwaJ9T2VGRDAbLTJ88";

// hashed password: strong_and_complicated
const PASSWORD_HASHED: &str = "$argon2id$v=19$m=19456,t=2,p=1$0XlThjIrqV/k9qYikOIlSw$AjZYlgq77E2JsafM+B9q2mp/TIChK8sy64uQyQZLk3A";

// ------------------------- POST /api/users/login -----------------------------
async fn login_empty_payload(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users/login").unwrap();
    let response = ctx.http_client.post(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!response.bytes().await.unwrap().is_empty());
}

async fn assert_invalid_login(ctx: &TestContext, login_payload: serde_json::Value, msg: &str) {
    let url = ctx.backend_url.join("/api/users/login").unwrap();

    let response = ctx
        .http_client
        .post(url)
        .json(&json!({ "user": login_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
    assert!(!response.bytes().await.unwrap().is_empty());
}

async fn login_user_email_issues(ctx: TestContext) {
    let cases = [
        (
            json!({
                "password": "strong_and_complicated",
                "captcha": "test",
            }),
            "email not provided",
        ),
        (
            json!({
                "email": 123,
                "password": "strong_and_complicated",
                "captcha": "test",
            }),
            "email is not a string",
        ),
        (
            json!({
                "email": "",
                "password": "strong_and_complicated",
                "captcha": "test",
            }),
            "email is empty string",
        ),
        (
            json!({
                "email": "rob.pike.com",
                "password": "strong_and_complicated",
                "captcha": "test",
            }),
            "email is not valid email",
        ),
    ];

    for (case, msg) in cases {
        assert_invalid_login(&ctx, case, msg).await;
    }
}

async fn login_user_password_issues(ctx: TestContext) {
    let cases = [
        (
            json!({
                "email": "rob.pike.com",
                "captcha": "test",
            }),
            "password not provided",
        ),
        (
            json!({
                "email": "rob.pike.com",
                "password": 123,
                "captcha": "test",
            }),
            "password is not a string",
        ),
        (
            json!({
                "email": "rob.pike.com",
                "password": "",
                "captcha": "test",
            }),
            "password is empty string",
        ),
        (
            json!({
                "email": "rob.pike.com",
                "password": "12345",
                "captcha": "test",
            }),
            "password is less than 12 characters",
        ),
    ];

    for (case, msg) in cases {
        assert_invalid_login(&ctx, case, msg).await;
    }
}

// login user with correct payload
async fn login_user(ctx: TestContext) {
    sqlx::query(
        r#"
        INSERT INTO users (email, username, password_hash, status)
        VALUES ($1, $2, $3, $4)
    "#,
    )
    .bind("rob.pike@gmail.com")
    .bind("rob.pike")
    .bind(PASSWORD_HASHED)
    .bind("ACTIVE")
    .execute(&ctx.db_pool)
    .await
    .expect("failed to insert test user");

    let login_payload = json!({
        "email": "rob.pike@gmail.com",
        "password": "strong_and_complicated",
        "captcha": "test"
    });

    let response = ctx
        .http_client
        .post(ctx.backend_url.join("/api/users/login").unwrap())
        .json(&json!({ "user": login_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.bytes().await.unwrap().is_empty());
}

mod tests {
    crate::async_test!(login_empty_payload);
    crate::async_test!(login_user_email_issues);
    crate::async_test!(login_user_password_issues);
    crate::async_test!(login_user);
}
