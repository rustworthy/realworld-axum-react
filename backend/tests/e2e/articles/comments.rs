use chrono::DateTime;
use reqwest::StatusCode;
use serde_json::{Value, json};
use url::Url;

use crate::utils::{TestContext, fake};

async fn create_comment(ctx: TestContext) {
    // let's try to create a comment without authentication
    // (we know that the article `whatever` is not there and that payload
    // is missing, but authentication check always comes first)
    let resp = ctx
        .http_client
        .post(
            ctx.backend_url
                .join("/api/articles/whatever/comments")
                .expect("valid url"),
        )
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // let's now ensure a user and retry the call from above, but
    // this time around let's prvide a valid token
    let user = fake::create_activated_user(&ctx).await;
    let resp = ctx
        .http_client
        .post(
            ctx.backend_url
                .join("/api/articles/whatever/comments")
                .expect("valid url"),
        )
        .json(&json!({"comment": {"body": ""}})) // empty string
        .bearer_auth(&user.token)
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // ok, let's now try an unethicated call with a perfectly valid payload
    // (reminder: there are no articles in the database just yet)
    let user = fake::create_activated_user(&ctx).await;
    let resp = ctx
        .http_client
        .post(
            ctx.backend_url
                .join("/api/articles/whatever/comments")
                .expect("valid url"),
        )
        .json(&json!({"comment": {"body": "There is a typo!"}}))
        .bearer_auth(&user.token)
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // let's create an article ...
    let slugs = fake::gen_articles(&ctx.backend_url, &user.token, 1, None).await;

    // ... and check the happy path
    let resp = ctx
        .http_client
        .post(
            ctx.backend_url
                .join(&format!("/api/articles/{}/comments", &slugs[0]))
                .expect("valid url"),
        )
        .json(&json!({"comment": {"body": "There is a typo!"}}))
        .bearer_auth(&user.token)
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::OK);

    let payload: Value = resp.json().await.expect("JSON payload");
    assert!(payload["comment"]["id"].is_string());
    assert!(
        DateTime::parse_from_rfc3339(payload["comment"]["createdAt"].as_str().unwrap()).is_ok()
    );
    assert!(
        DateTime::parse_from_rfc3339(payload["comment"]["updatedAt"].as_str().unwrap()).is_ok()
    );
    assert_eq!(payload["comment"]["body"], "There is a typo!");
    assert_eq!(payload["comment"]["author"]["username"], user.username);
    assert_eq!(payload["comment"]["author"]["bio"], user.bio);
    assert_eq!(
        payload["comment"]["author"]["image"]
            .as_str()
            .and_then(|input| Url::parse(input).ok()),
        user.image
    );
    assert_eq!(payload["comment"]["author"]["following"], false);
}

mod tests {
    crate::async_test!(create_comment);
}
