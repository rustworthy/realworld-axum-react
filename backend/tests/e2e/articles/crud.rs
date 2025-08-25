use crate::utils::TestContext;
use crate::utils::fake;
use reqwest::StatusCode;
use serde_json::{Value, json};

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

async fn create_article_payload_issues(ctx: TestContext) {
    let user = fake::create_activated_user(&ctx).await;

    let cases = [
        (
            json!({
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "title not provided",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "description not provided",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "tagList": ["language-design", "rust"]
            }),
            "body not provided",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety."
            }),
            "tagList not provided",
        ),
        (
            json!({
                "title": 123,
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "title is not a string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": 123,
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "description is not a string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": 123,
                "tagList": ["language-design", "rust"]
            }),
            "body is not a string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": "not-an-array"
            }),
            "tagList is not an array",
        ),
        (
            json!({
                "title": "",
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "title is empty string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": ["language-design", "rust"]
            }),
            "description is empty string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": "",
                "tagList": ["language-design", "rust"]
            }),
            "body is empty string",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": []
            }),
            "tagList is empty array",
        ),
        (
            json!({
                "title": "Type-Safe Programming Languages",
                "description": "Type systems and memory safety",
                "body": "Language design requires balancing expressiveness and safety.",
                "tagList": [123, "rust"]
            }),
            "tagList contains non-string elements",
        ),
    ];

    for (case, msg) in cases {
        let url = ctx.backend_url.join("/api/articles").unwrap();
        let response = ctx
            .http_client
            .post(url)
            .bearer_auth(&user.token)
            .json(&json!({ "article": case }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
    }

    // let's now create an article and let back-end generate a slug
    let title = "Type-Safe Programming Languages";
    let valid_article_details = json!({
        "title": title,
        "description": "Type systems and memory safety",
        "body": "Language design requires balancing expressiveness and safety.",
        "tagList": ["language-design", "rust", "mozilla"]
    });
    let response = ctx
        .http_client
        .post(ctx.backend_url.join("/api/articles").unwrap())
        .bearer_auth(&user.token)
        .json(&json!({ "article": valid_article_details }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // here is our slug
    let resp: Value = response.json().await.unwrap();
    let slug = resp
        .get("article")
        .unwrap()
        .as_object()
        .unwrap()
        .get("slug")
        .unwrap();
    assert_eq!(slug, "type-safe-programming-languages");

    // and so what happens if we try to create an article with
    // the name that give the same slug?
    let article_details = json!({
        "title": title.to_uppercase(),
        "description": "Type systems and memory safety",
        "body": "Language design requires balancing expressiveness and safety.",
        "tagList": ["language-design", "rust", "mozilla"]
    });
    let response = ctx
        .http_client
        .post(ctx.backend_url.join("/api/articles").unwrap())
        .bearer_auth(&user.token)
        .json(&json!({ "article": article_details }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let payload: Value = response.json().await.unwrap();
    let error = payload
        .get("errors")
        .unwrap()
        .as_object()
        .unwrap()
        .get("title")
        .unwrap()
        .as_array()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(error, "article with this title already exists");
}

mod tests {
    crate::async_test!(create_article_no_authentication);
    crate::async_test!(create_article_empty_payload);
    crate::async_test!(create_article_payload_issues);
}
