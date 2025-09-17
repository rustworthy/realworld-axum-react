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

async fn create_article_and_read_it(ctx: TestContext) {
    // try to read an article using slug "type-safe-programming-languages",
    // observe that the article is not there;
    let slug = "type-safe-programming-languages";
    let url = ctx
        .backend_url
        .join(&format!("/api/articles/{}", slug))
        .unwrap();
    let response = ctx.http_client.get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // check that the articles table is empty
    let article_count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM "articles""#)
        .fetch_one(&ctx.db_pool)
        .await
        .unwrap();
    assert_eq!(article_count, 0);

    // now, let's create an article with the title that we know will give that slug
    let user = fake::create_activated_user(&ctx).await;

    let title = "Type-Safe Programming Languages";
    let description = "Type systems and memory safety";
    let body = "Language design requires balancing expressiveness and safety.";
    let tags = ["language-design", "rust"];

    let article_details = json!({
        "title": title,
        "description": description,
        "body": body,
        "tagList": tags
    });
    let create_response = ctx
        .http_client
        .post(ctx.backend_url.join("/api/articles").unwrap())
        .bearer_auth(&user.token)
        .json(&json!({ "article": article_details }))
        .send()
        .await
        .unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    // let's now check that the articles table is not empty, there is one entry there
    let article_count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM "articles""#)
        .fetch_one(&ctx.db_pool)
        .await
        .unwrap();
    assert_eq!(article_count, 1);

    // parse response body and get the slug out of there, check that it is the
    // slug we initially used (that gave us 404) and use this slug to
    // retrieve the article
    let resp: Value = create_response.json().await.unwrap();
    let returned_slug = resp
        .get("article")
        .unwrap()
        .as_object()
        .unwrap()
        .get("slug")
        .unwrap()
        .as_str()
        .unwrap();
    assert_eq!(returned_slug, slug);
    let read_url = ctx
        .backend_url
        .join(&format!("/api/articles/{}", returned_slug))
        .unwrap();
    let read_response = ctx.http_client.get(read_url).send().await.unwrap();
    assert_eq!(read_response.status(), StatusCode::OK);

    // we can see the details of the article, including ...
    let resp: Value = read_response.json().await.unwrap();
    let article = resp.get("article").unwrap().as_object().unwrap();
    assert_eq!(article.get("slug").unwrap(), returned_slug);
    assert_eq!(article.get("title").unwrap(), title);
    assert_eq!(article.get("description").unwrap(), description);
    assert_eq!(article.get("body").unwrap(), body);
    assert_eq!(article.get("tagList").unwrap(), &json!(tags));
    assert_eq!(article.get("favoritesCount").unwrap(), 0);
    assert_eq!(article.get("favorited").unwrap(), false);

    // ... its author - the current user (reminder: usernames are unique)
    let author = article.get("author").unwrap().as_object().unwrap();
    assert_eq!(author.get("username").unwrap(), &user.username); // NB
    assert_eq!(author.get("bio").unwrap(), &user.bio);
    assert_eq!(author.get("following").unwrap(), false);
    assert_eq!(author.get("image").unwrap(), &Value::Null);
}

async fn delete_article(ctx: TestContext) {
    let user1 = fake::create_activated_user(&ctx).await;
    let user2 = fake::create_activated_user(&ctx).await;

    let article_details = json!({
        "title": "Delete article test",
        "description": "Delete article test",
        "body": "Delete artcile test",
        "tagList": ["test"]
    });
    let resp_payload = ctx // "delete-article-test"
        .http_client
        .post(ctx.backend_url.join("/api/articles").unwrap())
        .bearer_auth(&user1.token) // this will be first user's article
        .json(&json!({ "article": article_details }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    let slug = resp_payload
        .get("article")
        .unwrap()
        .as_object()
        .unwrap()
        .get("slug")
        .unwrap()
        .as_str()
        .unwrap();

    // let's first check that users cannot DELETE w/o authentication
    assert_eq!(
        ctx.http_client
            .delete(
                ctx.backend_url
                    .join("/api/articles/")
                    .unwrap()
                    .join(slug)
                    .unwrap()
            )
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );

    // now, we will use the token, but try to DELETE an article that
    // we know does not exist
    assert_eq!(
        ctx.http_client
            .delete(
                ctx.backend_url
                    .join("/api/articles/")
                    .unwrap()
                    .join("does-not-exist")
                    .unwrap()
            )
            .bearer_auth(&user2.token)
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );

    // ok, but what if our _authenticated_ second user now tries to DELETE
    // the first user's article?
    assert_eq!(
        ctx.http_client
            .delete(
                ctx.backend_url
                    .join("/api/articles/")
                    .unwrap()
                    .join(slug) // first user's article
                    .unwrap()
            )
            .bearer_auth(&user2.token) // second user's token
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN // not allowed
    );

    // let's make sure the author can delete their article
    assert_eq!(
        ctx.http_client
            .delete(
                ctx.backend_url
                    .join("/api/articles/")
                    .unwrap()
                    .join(slug) // first user's article
                    .unwrap()
            )
            .bearer_auth(&user1.token) // and first user's token
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NO_CONTENT // deletes just fine
    );

    // sanity: let's check that the article is not longer there
    assert_eq!(
        ctx.http_client
            .get(
                ctx.backend_url
                    .join("/api/articles/")
                    .unwrap()
                    .join(slug) // first user's article
                    .unwrap()
            )
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    )
}

mod tests {
    crate::async_test!(create_article_no_authentication);
    crate::async_test!(create_article_empty_payload);
    crate::async_test!(create_article_payload_issues);
    crate::async_test!(create_article_and_read_it);
    crate::async_test!(delete_article);
}
