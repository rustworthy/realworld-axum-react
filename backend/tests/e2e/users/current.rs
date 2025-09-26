use reqwest::{StatusCode, header};
use serde_json::{Value, json};
use crate::utils::{TestContext, extract_otp_from_html};

// --------------------------- PUT /api/user -----------------------------------

// update unauthenticated user
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

// register and login user
async fn login_user_to_update(ctx: &TestContext) -> String {
    // register new user
    let url = ctx.backend_url.join("/api/users").unwrap();
    let registration = json!({
        "username": "rob.pike",
        "email": "rob.pike@gmail.com",
        "password": "strong_and_complicated",
        "captcha": "test",
    });

    let _response = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": registration
        }))
        .send()
        .await
        .unwrap();

    // send email with OTP
    let otp_email_request: Value = ctx
        .mailer_server
        .received_requests()
        .await
        .expect("requests to have been received")
        .first()
        .expect("letter with OTP to have been sent")
        .body_json()
        .expect("JSON payload");

    // parse the OTP
    let html = otp_email_request
        .get("html")
        .expect("'html' field to be present in request payload")
        .as_str()
        .expect("html content to be a string");

    // extract otp from html
    let otp_sent = extract_otp_from_html(html);

    // now that we got our OTP, let's confirm the email
    let url = ctx.backend_url.join("/api/users/confirm-email").unwrap();
    let _response = ctx
        .http_client
        .post(url)
        .json(&json!({
            "user": {
                "otp": otp_sent,
                "captcha": "test",
            }
        }))
        .send()
        .await
        .unwrap();

    // login new user
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

    // return extracted token
    let response_body: Value = response.json().await.unwrap();
    let token = response_body["user"]["token"]
        .as_str()
        .expect("token to be present");

    token.to_string()
}

// test update with invalid payloads
async fn assert_invalid_update(ctx: &TestContext, update: serde_json::Value, msg: &str) {
    let token = login_user_to_update(&ctx).await;
    let url = ctx.backend_url.join("/api/user").unwrap();

    let response = ctx
        .http_client
        .put(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({ "user": update }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
    assert!(!response.bytes().await.unwrap().is_empty());
}

async fn update_user_issues(ctx: TestContext) {
    let cases = [
        (
            json!({
                "email": "rob.pike@@@gmail.com",
                "captcha": "test",
            }),
            "invalid email format",
        ),
        (
            json!({
                "username": 123,
                "captcha": "test",
            }),
            "username is not a string",
        ),
        (
            json!({
                "username": "",
                "captcha": "test",
            }),
            "username cannot be empty",
        ),
        (
            json!({
                "password": "123",
                "captcha": "test",
            }),
            "password should be at least 12 characters long",
        ),
    ];

    for (case, msg) in cases {
        assert_invalid_update(&ctx, case, msg).await;
    }
}

// test update with valid payload
async fn update_user_success(ctx: TestContext) {
    let token = login_user_to_update(&ctx).await;

    let update_payload = json!({
        "image": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRn8EPGHcF2cf6taBepWvRHYPs2n51Y6-7KpA&s",
        "username": "new rob.pike",
        "bio": "I am person, but I am not sure",
        "email": "rob.pike@gmail.com",
        "password": "NEW_strong_and_complicated_NEW",
        "captcha": "test"
    });

    let update_url = ctx.backend_url.join("/api/user").unwrap();

    let response = ctx
        .http_client
        .put(update_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({ "user": update_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.bytes().await.unwrap().is_empty());
}

mod tests {
    crate::async_test!(update_user_unauthenticated);
    crate::async_test!(update_user_issues);
    crate::async_test!(update_user_success);
}
