use chrono::DateTime;
use reqwest::StatusCode;
use serde_json::{Value, json};
use url::Url;

use crate::utils::{TestContext, fake};

async fn create_read_and_delete_comments(ctx: TestContext) {
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

    // let's imagine a user got authenticated, but then deleted ...
    let another_user = fake::create_activated_user(&ctx).await;
    sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(&another_user.username)
        .execute(&ctx.db_pool)
        .await
        .unwrap();

    // ... and try to post another comment
    let resp = ctx
        .http_client
        .post(
            ctx.backend_url
                .join(&format!("/api/articles/{}/comments", &slugs[0]))
                .expect("valid url"),
        )
        .json(&json!({"comment": {"body": "There is a typo!"}}))
        .bearer_auth(&another_user.token) // NB: token is still valid
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // let's "re-create" our second user and make it so they are following
    // the artcticle's author
    let another_user = fake::create_activated_user(&ctx).await;
    // at the time of writing this test, we do not have follow/unfollow
    // endpoints yet, ans so we are talking to db directly here
    sqlx::query(
        r#"
        INSERT INTO follows (followed_user_id, following_user_id)
        VALUES (
            (SELECT user_id FROM users WHERE username = $1),
            (SELECT user_id FROM users WHERE username = $2)
        );
        "#,
    )
    .bind(&user.username) // article and the sole comment author
    .bind(&another_user.username) // article and comment reader
    .execute(&ctx.db_pool)
    .await
    .unwrap();

    // let's list the comments w/o authentication first
    let resp = ctx
        .http_client
        .get(
            ctx.backend_url
                .join(&format!("/api/articles/{}/comments", &slugs[0]))
                .expect("valid url"),
        )
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::OK);
    let payload: Value = resp.json().await.expect("JSON payload");
    assert_eq!(payload["comments"].as_array().unwrap().len(), 1);
    assert!(payload["comments"][0]["body"].is_string());
    // the default for `following` is `false` when dealing
    // with unauthenticated requests to public routes
    assert!(
        !payload["comments"][0]["author"]["following"]
            .as_bool()
            .unwrap()
    );

    // if we now send an authenticated request, we should be able to see
    // that our another user is actually following the comment's author
    let resp = ctx
        .http_client
        .get(
            ctx.backend_url
                .join(&format!("/api/articles/{}/comments", &slugs[0]))
                .expect("valid url"),
        )
        .bearer_auth(&another_user.token)
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::OK);
    let payload: Value = resp.json().await.expect("JSON payload");
    assert_eq!(payload["comments"].as_array().unwrap().len(), 1);
    assert!(
        payload["comments"][0]["author"]["following"] // NB
            .as_bool()
            .unwrap()
    );

    // what if they now try to delete the comment
    let delete_comment_url = ctx
        .backend_url
        .join(&format!(
            "/api/articles/{}/comments/{}",
            &slugs[0],
            payload["comments"][0]["id"].as_str().unwrap()
        ))
        .expect("valid url");
    let resp = ctx
        .http_client
        .delete(delete_comment_url.clone())
        .bearer_auth(&another_user.token) // they never author this comment!
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // while the comment's author can delete their comment
    let resp = ctx
        .http_client
        .delete(delete_comment_url)
        .bearer_auth(&user.token)
        .send()
        .await
        .expect("http request to succeed");
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // finally, let's check db state
    let ncomments: i64 =
        sqlx::query_scalar(r#"SELECT COALESCE(COUNT(*), 0) AS "count!" FROM comments"#)
            .fetch_one(&ctx.db_pool)
            .await
            .unwrap();
    assert_eq!(ncomments, 0);
}

mod tests {
    crate::async_test!(create_read_and_delete_comments);
}
