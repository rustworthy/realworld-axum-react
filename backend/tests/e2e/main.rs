mod utils;

#[rocket::async_test]
async fn test1() {
    // arrange
    let ctx = utils::setup("test1").await;

    dbg!(&ctx.url);

    // act
    let _rocket = ctx.rocket.launch().await.unwrap();

    // assert
}

#[rocket::async_test]
async fn test2() {
    // arrange
    let ctx = utils::setup("test2").await;

    dbg!(&ctx.url);

    // act
    let _rocket = ctx.rocket.launch().await.unwrap();

    // assert
}

#[rocket::async_test]
async fn test3() {
    // arrange
    let ctx = utils::setup("test3").await;

    dbg!(&ctx.url);

    // act
    let _rocket = ctx.rocket.launch().await.unwrap();

    // assert
}
