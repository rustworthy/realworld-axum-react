use crate::utils::{TestContext, fake};
use reqwest::{StatusCode};
use serde_json::{Value, json};

async fn follow_user_profile(ctx: TestContext) {
    let user1 = fake::create_activated_user(&ctx).await;

    // ---------- read non-existent user ----------
    let url_path = format!("/api/profiles/{}", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // ---------- create user2 and fetch their profile ----------
    let user2 = fake::create_activated_user(&ctx).await;
    let url_path = format!("/api/profiles/{}", user2.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // ---------- check if user1 following user2 ----------
    let response_json: Value = response.json().await.unwrap();
    let following = response_json
        .get("profile")
        .and_then(|p| p.get("following"))
        .and_then(|f| f.as_bool())
        .expect("Missing or invalid 'following' field");

    assert!(!following, "Expected 'following' to be false, but got true");

    // ---------- try to follow user2 without auth and get 401 ----------
    let url_path = format!("/api/profiles/{}/follow", user2.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .post(url)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // ---------- user1 follow user2 with auth ----------
    let url_path = format!("/api/profiles/{}/follow", user2.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .post(url)
        .bearer_auth(user1.token.clone())
        .send()
        .await
        .unwrap();

    let expected_json = json!({
        "profile": {
            "username": user2.username,
            "bio": user2.bio,
            "image": user2.image,
            "following": true,
        }
    });
    let response_json: Value = response.json().await.unwrap();
    assert_eq!(response_json, expected_json);

    // ---------- user1 read profile of user2 without token ----------
    let url_path = format!("/api/profiles/{}", user2.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .get(url)
        .send()
        .await
        .unwrap();

    let expected_json = json!({
        "profile": {
            "username": user2.username,
            "bio": user2.bio,
            "image": user2.image,
            "following": false,
        }
    });
    let response_json: Value = response.json().await.unwrap();
    assert_eq!(response_json, expected_json);

    // ---------- user1 unfollow user2 ----------
    let url_path = format!("/api/profiles/{}/follow", user2.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .delete(url)
        .bearer_auth(user1.token.clone())
        .send()
        .await
        .unwrap();

    let expected_json = json!({
        "profile": {
            "username": user2.username,
            "bio": user2.bio,
            "image": user2.image,
            "following": false,
        }
    });
    let response_json: Value = response.json().await.unwrap();
    assert_eq!(response_json, expected_json);

    // ---------- user1 try to follow itself ----------
    let url_path = format!("/api/profiles/{}/follow", user1.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .post(url)
        .bearer_auth(user1.token.clone())
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

mod tests {
    crate::async_test!(follow_user_profile);
}
