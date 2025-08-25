use super::utils::fake;
use crate::utils::TestContext;
use reqwest::StatusCode;

// ----------------------------- CREATE --------------------------------------
async fn create_article_no_authentication(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/articles").unwrap();
    let response = ctx.http_client.post(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

async fn create_article_empty_payload(ctx: TestContext) {
    let user = fake::create_activated_user(&ctx).await;
    let url = ctx.backend_url.join("/api/articles").unwrap();
    let response = ctx
        .http_client
        .post(url)
        .bearer_auth(user.token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

mod tests {
    crate::async_test!(create_article_no_authentication);
    crate::async_test!(create_article_empty_payload);
}
