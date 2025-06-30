use crate::utils::setup;
use rocket::http::Status;

#[rocket::async_test]
async fn get_current_user_unauthenticated() {
    // arrange
    let ctx = setup("get_current_user").await;

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
