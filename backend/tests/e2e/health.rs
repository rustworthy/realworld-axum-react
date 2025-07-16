use crate::utils::setup;
use rocket::http::{ContentType, Status};

#[tokio::test]
async fn healthz_endpoint() {
    // arrange
    let ctx = setup("healthz_endpoint").await;

    // act
    let response = ctx.client.get("/healthz").dispatch().await;

    // assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.into_string().await.unwrap().contains("version"));
}
