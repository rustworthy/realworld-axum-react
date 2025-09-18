use crate::utils::TestContext;
use reqwest::StatusCode;

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

mod tests {
    crate::async_test!(list_articles_query_issues);
}
