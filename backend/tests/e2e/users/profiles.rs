use crate::utils::{TestContext, fake};
use reqwest::{StatusCode};
use serde_json::{Value, json};

// get user profile issues
async fn get_user_profile_issues(ctx: TestContext) {
    let url_path = format!("/api/profiles/{}", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.get(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// follow user profile issues
async fn follow_user_profile_issues(ctx: TestContext) {
    // check follow if unauthorized and with non existent username
    let url_path = format!("/api/profiles/{}/follow", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.post(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // check follow if authorized and with non existent username
    let user = fake::create_activated_user(&ctx).await;

    let url_path = format!("/api/profiles/{}/follow", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .post(url)
        .bearer_auth(user.token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// unfollow user profile issues
async fn unfollow_user_profile_issues(ctx: TestContext) {
    // check unfollow if unauthorized and with non existent username
    let url_path = format!("/api/profiles/{}/follow", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.delete(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // check unfollow if authorized and with non existent username
    let user = fake::create_activated_user(&ctx).await;

    let url_path = format!("/api/profiles/{}/follow", "non_existent_username");
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .delete(url)
        .bearer_auth(user.token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// get user profile
async fn get_user_profile(ctx: TestContext) {
    let user = fake::create_activated_user(&ctx).await;

    let url_path = format!("/api/profiles/{}", user.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx.http_client.get(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let expected_json = json!({
        "profile": {
            "username": user.username,
            "bio": user.bio,
            "image": user.image,
            "following": false,
        }
    });
    let response_json: Value = response.json().await.unwrap();

    assert_eq!(response_json, expected_json);
}

// follow user profile
async fn follow_user_profile(ctx: TestContext) {
    let target = fake::create_activated_user(&ctx).await;
    let follower = fake::create_activated_user(&ctx).await;

    let url_path = format!("/api/profiles/{}/follow", target.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .post(url)
        .bearer_auth(follower.token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let expected_json = json!({
        "profile": {
            "username": target.username,
            "bio": target.bio,
            "image": target.image,
            "following": true,
        }
    });
    let response_json: Value = response.json().await.unwrap();

    assert_eq!(response_json, expected_json);
}

// unfollow user profile
async fn unfollow_user_profile(ctx: TestContext) {
    let target = fake::create_activated_user(&ctx).await;
    let follower = fake::create_activated_user(&ctx).await;

    let url_path = format!("/api/profiles/{}/follow", target.username);
    let url = ctx.backend_url.join(&url_path).unwrap();

    let response = ctx
        .http_client
        .delete(url)
        .bearer_auth(follower.token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let expected_json = json!({
        "profile": {
            "username": target.username,
            "bio": target.bio,
            "image": target.image,
            "following": false,
        }
    });
    let response_json: Value = response.json().await.unwrap();

    assert_eq!(response_json, expected_json);
}

mod tests {
    crate::async_test!(get_user_profile_issues);
    crate::async_test!(follow_user_profile_issues);
    crate::async_test!(unfollow_user_profile_issues);
    crate::async_test!(get_user_profile);
    crate::async_test!(follow_user_profile);
    crate::async_test!(unfollow_user_profile);
}
