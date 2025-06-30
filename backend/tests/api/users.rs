use crate::utils::setup;
use rocket::http::{Header, Status};

// This token has been signed with using a secret in our `.env.example`, while
// for each of our tests we are launching a dedicated rocker application with a
// dedicated random secret key, and so we expect the back-end to reject us
const TEST_JWT_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyNWY3NTMzNy1hNWUzLTQ0YjEtOTdkNy02NjUzY2EyM2U5ZWUiLCJpYXQiOjE3NTEzMTE5NzksImV4cCI6MTc1MTkxNjc3OX0.QJXG34zRbMLin8JUr-BBbwOSQWwaJ9T2VGRDAbLTJ88";

#[rocket::async_test]
async fn get_current_user_no_token() {
    // arrange
    let ctx = setup("get_current_user_no_token").await;

    // act
    let response = ctx.client.get("/api/user").dispatch().await;

    // assert
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        response.headers().get_one("WWW-Authenticate").unwrap(),
        "Token"
    );
    assert!(response.body().is_none());
}

#[rocket::async_test]
async fn get_current_user_invalid_token() {
    // arrange
    let ctx = setup("get_current_user_invalid_token").await;

    // act
    let mut request = ctx.client.get("/api/user");
    request.add_header(Header::new(
        "Authorization",
        format!("Token {}", TEST_JWT_TOKEN),
    ));
    let response = request.dispatch().await;

    // assert
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        response.headers().get_one("WWW-Authenticate").unwrap(),
        "Token"
    );
    assert!(response.body().is_none());
}
