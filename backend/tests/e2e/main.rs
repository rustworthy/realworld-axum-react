#![cfg(feature = "e2e-test")]

mod utils;

use fantoccini::Locator;
use std::time::Duration;

const WAIT_TIMEOUT: Duration = Duration::from_secs(5);

async fn has_link_to_github_repo() {
    // arrange
    let ctx = utils::setup("leads_to_github_repo").await;

    // act
    ctx.client.goto(&ctx.url).await.unwrap();
    let elem = ctx
        .client
        .wait()
        .at_most(WAIT_TIMEOUT)
        .for_element(Locator::LinkText("Fork on GitHub"))
        .await
        .unwrap();
    elem.follow().await.unwrap();

    // assert
    assert_eq!(
        ctx.client.current_url().await.unwrap().domain(),
        Some("github.com"),
    );

    // clean up
    ctx.client.close().await.ok();
    ctx.handle.abort();
}

async fn homepage_contains_project_name() {
    // arrange
    let ctx = utils::setup("test2").await;

    // act
    ctx.client.goto(&ctx.url).await.unwrap();
    let h1 = ctx
        .client
        .wait()
        .at_most(WAIT_TIMEOUT)
        .for_element(Locator::Css("h1"))
        .await
        .unwrap();

    // assert
    assert_eq!(h1.text().await.unwrap(), "conduit",);

    // clean up
    ctx.client.close().await.ok();
    ctx.handle.abort();
}

mod tests {
    super::async_test!(has_link_to_github_repo);
    super::async_test!(homepage_contains_project_name);
}
