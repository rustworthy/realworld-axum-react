use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn healthz_endpoint() {
    // arrange
    let rocket = realworld_rocket_react::construct_rocket(
        "postgres://postgres:postgres@localhost:5432/realworld-rocket-react".into(),
        true,
    );
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket application");

    // act
    let response = client.get("/healthz").dispatch().await;

    // assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.into_string().await.unwrap().contains("version"));
}
