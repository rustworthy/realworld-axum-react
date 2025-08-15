use reqwest::{StatusCode, header};
use serde_json::{Value, json};
use sqlx::Row as _;
use url::Url;

use crate::utils::TestContext;

// This token has been signed with using a secret in our `.env.example`, while
// for each of our tests we are launching a dedicated rocker application with a
// dedicated random secret key, and so we expect the back-end to reject us
const TEST_JWT_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyNWY3NTMzNy1hNWUzLTQ0YjEtOTdkNy02NjUzY2EyM2U5ZWUiLCJpYXQiOjE3NTEzMTE5NzksImV4cCI6MTc1MTkxNjc3OX0.QJXG34zRbMLin8JUr-BBbwOSQWwaJ9T2VGRDAbLTJ88";

// ----------------------------- REGISTER --------------------------------------
async fn create_user_empty_payload(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users").unwrap();
    let response = ctx.http_client.post(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!response.bytes().await.unwrap().is_empty());
}

async fn create_user_empty_username(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users").unwrap();
    let registration = json!({
        "email": "rob.pike@gmail.com",
        "password": "qwerty",
    });

    let response = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": registration
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!response.bytes().await.unwrap().is_empty());
}

async fn create_user_empty_email(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users").unwrap();
    let registration = json!({
        "username": "rob.pike",
        "password": "qwerty",
    });

    let response = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": registration
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!response.bytes().await.unwrap().is_empty());
}


// --------------------------- CONFIRM EMAIL -----------------------------------
async fn confirm_email_address(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users").unwrap();
    let registration = json!({
        "username": "rob.pike",
        "email": "rob.pike@gmail.com",
        "password": "qwerty",
    });

    let response: Value = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": registration
        }))
        .send()
        .await
        .expect("request to have succeeded")
        .json()
        .await
        .expect("valid JSON in response");

    let user_row = sqlx::query(r#"SELECT * FROM "users""#)
        .fetch_all(&ctx.db_pool)
        .await
        .expect("valid query");

    assert_eq!(user_row.len(), 1);
    let status: &str = user_row[0].get("status");
    assert_eq!(status, "EMAIL_CONFIRMATION_PENDING");


    let otp_rows = sqlx::query(r#"SELECT * FROM "confirmation_tokens""#)
        .fetch_all(&ctx.db_pool)
        .await
        .expect("valid query");

    assert_eq!(otp_rows.len(), 1);
    let purpose: &str = otp_rows[0].get("purpose");
    assert_eq!(purpose, "EMAIL_CONFIRMATION");

    let otp_stored: &str = otp_rows[0].get("token");

    let otp_email_request: Value = ctx
        .mailer_server
        .received_requests()
        .await
        .expect("requests to have been received")
        .first()
        .expect("letter with OTP to have been sent")
        .body_json()
        .expect("JSON payload");

    assert_eq!(
        otp_email_request
            .get("to")
            .expect("'to' field to be present in request payload")
            .as_array()
            .expect("array of addresses")
            .first()
            .expect("minimum one address"),
        "rob.pike@gmail.com"
    );

    let html = otp_email_request
        .get("html")
        .expect("'html' field to be present in request payload")
        .as_str()
        .expect("html content to be a string");

    let finder = linkify::LinkFinder::new();
    let links: Vec<_> = finder.links(html).collect();
    let otp_link: Url = links[1].as_str().parse().expect("value URL");
    let otp_sent = otp_link
        .query_pairs()
        .find(|(key, _)| key == "otp")
        .map(|(_, otp)| otp)
        .expect("OTP as query string parameter");
    assert_eq!(otp_sent, otp_stored);

    let url = ctx.backend_url.join("/api/users/confirm-email").unwrap();
    let response: Value = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": {
                "otp": otp_sent
            }
        }))
        .send()
        .await
        .expect("request to have succeeded")
        .json()
        .await
        .expect("valid JSON in response");

    let user = response
        .get("user")
        .expect("all user management endpoints to have 'user' as the root key")
        .as_object()
        .expect("user details JSON object");
    assert!(user.get("token").is_some());

    let user_row = sqlx::query(r#"SELECT * FROM "users""#)
        .fetch_all(&ctx.db_pool)
        .await
        .expect("valid query");
    assert_eq!(user_row.len(), 1);
    let status: &str = user_row[0].get("status");
    assert_eq!(status, "ACTIVE");
}

// ------------------------- POST /api/users/login -----------------------------
async fn login_empty_payload(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/users/login").unwrap();
    let response = ctx.http_client.post(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!response.bytes().await.unwrap().is_empty());
}

// ---------------------------- GET /api/user ----------------------------------
async fn get_current_user_no_token(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/user").unwrap();
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
    let url = ctx.backend_url.join("/api/user").unwrap();
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
    let url = ctx.backend_url.join("/api/user").unwrap();
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
    crate::async_test!(create_user_empty_username);
    crate::async_test!(create_user_empty_email);
    crate::async_test!(confirm_email_address);
    crate::async_test!(login_empty_payload);
    crate::async_test!(get_current_user_no_token);
    crate::async_test!(get_current_user_invalid_token);
    crate::async_test!(update_user_unauthenticated);
}
