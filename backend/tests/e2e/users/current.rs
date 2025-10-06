use crate::utils::{TestContext, fake};
use reqwest::{StatusCode, header};
use serde_json::{Value, json};

// --------------------------- PUT /api/user -----------------------------------

// update unauthenticated user
async fn update_user_unauthenticated(ctx: TestContext) {
    let url = ctx.backend_url.join("/api/user").unwrap();
    let response = ctx.http_client.put(url).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get(header::WWW_AUTHENTICATE).unwrap(),
        "Bearer"
    );
    assert!(response.bytes().await.unwrap().is_empty());
}

// test update with invalid payloads
async fn update_user_issues(ctx: TestContext) {
    let cases = [
        (
            json!({
                "email": "rob.pike@@@gmail.com",
                "captcha": "test",
            }),
            "invalid email format",
        ),
        (
            json!({
                "username": 123,
                "captcha": "test",
            }),
            "username is not a string",
        ),
        (
            json!({
                "username": "",
                "captcha": "test",
            }),
            "username cannot be empty",
        ),
        (
            json!({
                "password": "123",
                "captcha": "test",
            }),
            "password should be at least 12 characters long",
        ),
    ];

    let url = ctx.backend_url.join("/api/user").unwrap();

    for (case, msg) in cases {
        let user = fake::create_activated_user(&ctx).await;

        let response = ctx
            .http_client
            .put(url.clone())
            .bearer_auth(user.token)
            .json(&json!({ "user": case }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
        assert!(!response.bytes().await.unwrap().is_empty());
    }
}

// test update user with the same username and email
async fn update_user_with_duplicate_data(ctx: TestContext) {
    let user1 = fake::create_activated_user(&ctx).await;
    let user2 = fake::create_activated_user(&ctx).await;
    let url = ctx.backend_url.join("/api/user").unwrap();

    // check with the same username
    let update_payload = json!({
        "username": user2.username,
    });

    let response = ctx
        .http_client
        .put(url.clone())
        .bearer_auth(&user1.token)
        .json(&json!({ "user": update_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "duplicate username"
    );

    // check with the same email
    let update_payload = json!({
        "email": user2.email,
    });

    let response = ctx
        .http_client
        .put(url)
        .bearer_auth(user1.token)
        .json(&json!({ "user": update_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "duplicate email"
    );
}

// previously set image can be removed
async fn remove_previous_image(ctx: TestContext) {
    let user: fake::UserDetails = fake::create_activated_user(&ctx).await;
    let url = ctx.backend_url.join("/api/user").unwrap();

    // set new image
    let update_payload = json!({
        "image": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRn8EPGHcF2cf6taBepWvRHYPs2n51Y6-7KpA&s",
    });

    let response = ctx
        .http_client
        .put(url.clone())
        .bearer_auth(&user.token)
        .json(&json!({ "user": update_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // remove previous image, set null
    let remove_payload = json!({"image": Value::Null});

    let response = ctx
        .http_client
        .put(url.clone())
        .bearer_auth(&user.token)
        .json(&json!({ "user": remove_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // check image from db
    let image: Option<String> = sqlx::query_scalar(
        r#"
        SELECT image FROM users WHERE users.username = $1 
        "#,
    )
    .bind(&user.username)
    .fetch_one(&ctx.db_pool)
    .await
    .unwrap();

    assert!(
        image.is_none() || image.as_deref() == Some(""),
        "expect image to be NULL in db; got: {image:?}"
    )
}

// test update with valid payload
async fn update_user_success(ctx: TestContext) {
    let user = fake::create_activated_user(&ctx).await;
    let url = ctx.backend_url.join("/api/user").unwrap();

    let update_payload = json!({
        "image": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRn8EPGHcF2cf6taBepWvRHYPs2n51Y6-7KpA&s",
        "username": "new rob.pike",
        "bio": "I am person, but I am not sure",
        "email": "rob.pike@gmail.com",
        "password": "NEW_strong_and_complicated_NEW",
        "captcha": "test"
    });

    let response = ctx
        .http_client
        .put(url)
        .bearer_auth(user.token)
        .json(&json!({ "user": update_payload }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = response.json().await.unwrap();

    let updated_user = body.get("user").unwrap();

    let mut expected_user = update_payload.clone();
    expected_user.as_object_mut().unwrap().remove("password");
    expected_user.as_object_mut().unwrap().remove("captcha");

    for (k, v) in expected_user.as_object().unwrap() {
        assert_eq!(updated_user.get(k).unwrap(), v, "Mismatch at field {k}")
    }
}

mod tests {
    crate::async_test!(update_user_unauthenticated);
    crate::async_test!(update_user_issues);
    crate::async_test!(update_user_with_duplicate_data);
    crate::async_test!(remove_previous_image);
    crate::async_test!(update_user_success);
}
