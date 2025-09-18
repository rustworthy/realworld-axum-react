#![allow(unused)]

use super::Article;
use crate::http::errors::Error;
use crate::state::AppContext;
use axum::Json;
use axum::extract::rejection::{FailedToDeserializeQueryString, QueryRejection};
use axum::extract::{Query, State};
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use validator_derive::Validate;

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
    _ctx: State<Arc<AppContext>>,
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

    Ok(Json(ArticlesList {
        articles: vec![],
        count: 0,
    }))
}
