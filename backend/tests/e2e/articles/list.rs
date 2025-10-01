use crate::utils::{TestContext, fake};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;

async fn list_articles_query_issues(ctx: TestContext) {
    let cases = [
        (&[("limit", -1)], "limit negative"),
        (&[("limit", 1001)], "limit exceeded"),
        (&[("offset", -1)], "offset negative"),
    ];
    for (q, msg) in cases {
        let resp = ctx
            .http_client
            .get(ctx.backend_url.join("/api/articles").unwrap())
            .query(q)
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
    }

    // just to pin it here as currently allowed case: they can set limit
    // to `0`, maybe they are only interested in the number of articles
    // matching a specific query, who knows ¯\_(ツ)_/¯
    let resp = ctx
        .http_client
        .get(ctx.backend_url.join("/api/articles").unwrap())
        .query(&[("limit", 0)])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

async fn list_articles_with_limit_and_offset(ctx: TestContext) {
    // create three users ...
    let user1 = fake::create_activated_user(&ctx).await;
    let user2 = fake::create_activated_user(&ctx).await;
    let user3 = fake::create_activated_user(&ctx).await;
    let users = [&user1, &user2, &user3];

    // ... and make them authors
    let slugs_user1 = fake::gen_articles(
        &ctx.backend_url,
        &user1.token,
        50,
        Some(&["art", "modern", "oil"]), // note the "art" tag here ...
    )
    .await;
    let slugs_user2 = fake::gen_articles(
        &ctx.backend_url,
        &user2.token,
        30,
        Some(&["automotive", "racing"]),
    )
    .await;
    let slugs_user3 = fake::gen_articles(
        &ctx.backend_url,
        &user3.token,
        20,
        Some(&["programming", "art"]), // ... and here!
    )
    .await;

    // collect all of the slugs that are currently in the system
    let mut all_slugs = Vec::with_capacity(100);
    all_slugs.extend_from_slice(&slugs_user1);
    all_slugs.extend_from_slice(&slugs_user2);
    all_slugs.extend_from_slice(&slugs_user3);

    // let's first check the case where we are not specifying any offset or
    // limit, i.e. all the articles in the database are satisfying the filtering
    // as there is no filtering
    let (articles, count) = list_articles::<[(); 0]>(&ctx, &[]).await;
    assert_eq!(count, 100);
    // we have not provided the `limit` in the query and so the default
    // limit (20) has been applied
    assert_eq!(articles.len(), 20);

    // let's now specify an offset and limit; in the next test case, we are
    // skipping the first 90 entries and there are only 100 entries in
    // the database, we expect 10 articles to be returned, but overrall 100
    // matching the query (since, again, we are not asking for any filtering
    // in our query)
    let (articles, count) = list_articles(&ctx, &[("offset", 90), ("limit", 20)]).await;
    assert_eq!(count, 100);
    assert_eq!(articles.len(), 10);

    // let's now specify an offset that is larger than the available
    // articles count
    let (articles, count) = list_articles(&ctx, &[("offset", 200)]).await;
    assert_eq!(articles.len(), 0);
    assert_eq!(count, 100); // still 100

    // let's also pin here the case where we are allowing to set
    // the number of returned articles to 0, so that if we change
    // this logic, there is a flag in the tests
    let (articles, count) = list_articles(&ctx, &[("limit", 0)]).await;
    assert_eq!(articles.len(), 0);
    assert_eq!(count, 100); // still 100

    // let's only fetch the articles of a particular author
    let (articles, count) = list_articles(
        &ctx,
        &[("author", user2.username.as_str()), ("limit", "10")],
    )
    .await;
    // we set the limit to 10, but there are in total 30 articles authored
    // by a person with this username (we got 3 pages essentially)
    assert_eq!(articles.len(), 10);
    assert_eq!(count, 30);

    // let's also check filtering by tag, and by a cobination of tag
    // and author (to test that _all_ the filters are getting applied)
    let (articles, count) = list_articles(&ctx, &[("tag", "art")]).await;
    assert_eq!(articles.len(), 20); // default batch
    assert_eq!(count, 70); // user1's articles + user3's articles
    let (articles, count) =
        list_articles(&ctx, &[("tag", "art"), ("author", user3.username.as_str())]).await;
    assert_eq!(articles.len(), 20); // default batch
    assert_eq!(count, 20); // user3's articles only

    // finally, let's fetch a couple of articles and inspect
    let (articles, count) = list_articles(&ctx, &[("limit", 2)]).await;
    assert_eq!(articles.len(), 2);
    assert_eq!(count, 100);
    let article = articles[0].as_object().unwrap();
    assert!(all_slugs.contains(&article.get("slug").unwrap().as_str().unwrap().to_owned()));
    assert!(article.get("title").unwrap().is_string());
    assert!(article.get("description").unwrap().is_string());
    assert!(article.get("body").unwrap().is_string());
    assert!(article.get("tagList").unwrap().is_array());
    assert!(article.get("createdAt").unwrap().is_string());
    assert!(article.get("updatedAt").unwrap().is_string());
    // we are sending unauthenticated requests and so we `favorited`
    // field will default to `false`
    assert_eq!(article.get("favorited").unwrap(), false);
    assert_eq!(article.get("favoritesCount").unwrap(), 0); // default

    // let's find out which one of our seeded users authored this article
    let author = article.get("author").unwrap().as_object().unwrap();
    let user = users
        .iter()
        // reminder: usernames are unique in the system
        .find(|u| u.username == author.get("username").unwrap().as_str().unwrap())
        .unwrap();
    assert_eq!(author.get("bio").unwrap(), &user.bio);
    // again, we are sending unauthenticated requests, but even when we
    // authenticate this will still be `false`, since users cannot follow
    // themselves (they can though like their own articles)
    assert_eq!(author.get("following").unwrap(), false);
    assert_eq!(author.get("image").unwrap(), &Value::Null);

    // just one last check, let's verify that the articles are sorted
    // using the value of `created_at` in descending order
    let first_article_created_at: DateTime<Utc> = articles[0]
        .get("createdAt")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let second_article_created_at: DateTime<Utc> = articles[1]
        .get("createdAt")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    // i.e. the first articles is a more recent one (unless they were
    // pulished at the same time which is less likely but _is_ possible)
    assert!(first_article_created_at >= second_article_created_at);
}

/// List articles given the query.
///
/// A local helper, that will send a list articles request with the given query
/// and return a list of articles (as serde_json::Value) and total count.
async fn list_articles<T>(ctx: &TestContext, query: &T) -> (Vec<Value>, u64)
where
    T: Serialize,
{
    let resp = ctx
        .http_client
        .get(ctx.backend_url.join("/api/articles").unwrap())
        .query(query)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let payload: Value = resp.json().await.unwrap();
    let count = payload
        .get("articlesCount")
        .unwrap()
        .as_number()
        .unwrap()
        .as_u64()
        .unwrap();
    let articles = payload
        .get("articles")
        .unwrap()
        .as_array()
        .unwrap()
        .to_owned();
    (articles, count)
}

mod tests {
    crate::async_test!(list_articles_query_issues);
    crate::async_test!(list_articles_with_limit_and_offset);
}
