#![allow(unused)]

use super::Article;
use crate::http::errors::Error;
use crate::http::routes::articles::{self, Author};
use crate::http::routes::users::utils::parse_image_url;
use crate::state::AppContext;
use axum::Json;
use axum::extract::rejection::{FailedToDeserializeQueryString, QueryRejection};
use axum::extract::{Query, State};
use sqlx::Acquire;
use std::sync::Arc;
use tower_http::follow_redirect::policy::PolicyExt;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use validator_derive::Validate;

const DEFAULT_OFFSET: usize = 0;
const DEFAULT_LIMIT: usize = 20;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArticlesList {
    /// List of articles.
    articles: Vec<Article>,

    // Number of articles matching the query.
    #[serde(rename = "articlesCount")]
    #[schema(examples(1))]
    count: usize,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub(crate) struct ListQuery {
    /// Filter articles by tag.
    #[param(nullable = false, example = "Compilers")]
    tag: Option<String>,

    /// Filter articles by author (username).
    #[param(nullable = false, example = "timClicks")]
    author: Option<String>,

    /// Filter articles favorited by user (username).
    #[param(nullable = false, example = "rob.pike")]
    favorited: Option<String>,

    /// Limit number of returned articles.
    #[param(nullable = false, default = 20, maximum = 1000)]
    #[validate(range(max = 1000, message = "limit too large"))]
    limit: Option<usize>,

    /// Offset/skip number of articles.
    #[param(nullable = false, default = 0)]
    offset: Option<usize>,
}

/// List articles.
#[utoipa::path(
    get,
    path = "",
    tags = ["Articles"],
    params(ListQuery),
    responses(
        (status = 200, description = "Articles list successfully retrieved", body = [ArticlesList]),
        (status = 500, description = "Internal server error."),
    ),
)]
#[instrument(name = "LIST ARTICLES", skip_all)]
pub async fn list_articles(
    ctx: State<Arc<AppContext>>,
    // we are using `Result` here and unwrapping "manually" so that our error
    // is returned and then turned into a response with the status code that the
    // spec dictates (422) rather than axum's default 400 for `QueryRejection`;
    // see how are are implementing `From<QueryRejection>` for our Error in
    // `http::errors` module; note that we are doing the same with other extractors
    // fir the same reason, see for example how we are extracting article details
    // in `create_article` handler in `http::routes::articles::crud` module;
    q: Result<Query<ListQuery>, QueryRejection>,
) -> Result<Json<ArticlesList>, Error> {
    let Query(q) = q?;
    q.validate()?;

    let resp = sqlx::query!(
        r#"
        SELECT
            coalesce(count(*) OVER(), 0) "count!",
            article.slug,
            article.title,
            article.description,
            article.tags,
            article.created_at,
            article.updated_at,
            author.username as "author_username",
            author.bio as "author_bio",
            author.image as "author_image"
        FROM
            "articles" article JOIN "users" author USING (user_id)
        WHERE
            ($1::text IS NULL OR author.username = $1::text) AND
            ($2::text IS NULL OR article.tags @> ARRAY[$2::text])
        ORDER BY article.created_at DESC
        OFFSET $3
        LIMIT $4
    "#,
        q.author,
        q.tag,
        q.offset.unwrap_or(DEFAULT_OFFSET) as i64,
        q.limit.unwrap_or(DEFAULT_LIMIT) as i64,
    )
    .fetch_all(&ctx.db)
    .await?;

    let payload = if resp.is_empty() {
        // no rows can mean that:
        //  - no rows satisfying the filter (which is fine and we could simply
        //  return ArtcilesList{ articles: vec![], acount 0}), but there also
        //  at least two other reasons for zero rows returned:
        //  - `offset` greater than or equal to articles count or ...
        //  - `limit` is set to 0
        //  we are doing an extra db call without limit and offset
        //  to get the count of articles matching the filter;
        //
        //  there is a chance that an article matching query gets inserted
        //  and committed into the table after the initial query and prior to
        //  the following one, but with the default isolation level `READ COMMITTED`,
        //  even if we create a transaction ...
        //  ```
        //  let tx = ctx.db.begin().await?;
        //  ```
        //  ... and use the transaction as query executor we still can hit
        //  this corner case where we are returning 0 articles but also saying
        //  that there is, say, 1 article matching the query; we are allowing
        //  this case as not that likely to actually happen, but should keep it mind
        let count = sqlx::query_scalar!(
            r#"
            SELECT
                coalesce(count(*), 0) "count!"
            FROM
                articles JOIN users USING (user_id)
            WHERE
                ($1::text IS NULL OR username = $1::text) AND
                ($2::text IS NULL OR tags @> ARRAY[$2::text])
            "#,
            q.author,
            q.tag
        )
        .fetch_one(&ctx.db)
        .await?;
        ArticlesList {
            articles: vec![],
            count: count as usize,
        }
    } else {
        let count = resp[0].count as usize;
        let mut articles = Vec::with_capacity(resp.len());
        for item in resp {
            let article = Article {
                slug: item.slug,
                title: item.title,
                // as per the spec, to get the article's body, they need to query
                // a dedicated endpoint (`/api/articles/{slug}`); we could also
                // create a dedicated struct (say, ArticleListItem) omiting
                // the `body` field to not confuse out API user, but since
                // we are currently the only consumer of this API and provided
                // we have this documented via Open API UI (Scalar), it's fine
                // to re-use the "core" articles endpoints struct
                body: String::default(),
                description: item.description,
                tags: item.tags,
                created_at: item.created_at,
                updated_at: item.updated_at.unwrap_or(item.created_at),
                favorited: false, // TODO: update once supported for authed user is added
                favorited_count: 0, // TODO: upadte once "favorites" table is added
                author: {
                    Author {
                        username: item.author_username,
                        bio: item.author_bio,
                        image: parse_image_url(item.author_image.as_deref())?,
                        following: false, // TODO: upd once authed user case is added
                    }
                },
            };
            articles.push(article);
        }
        ArticlesList { articles, count }
    };
    Ok(Json(payload))
}
