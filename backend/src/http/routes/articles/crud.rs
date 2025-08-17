use super::{Article, ArticlePayload};
use crate::{
    http::{errors::Validation, extractors::UserID},
    state::AppContext,
};
use axum::extract::State;
use std::sync::Arc;

/// Create new article.
///
/// This will create register a new article in the database assigning it a slug,
/// which uniquely identifies it among other articles and can used to fetch it.
#[utoipa::path(
    post,
    path = "",
    tags = ["Articles"],
    responses(
        (status = 201, description = "Article successfully created", body = ArticlePayload<Article>),
        (status = 422, description = "Missing or invalid article attributes", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(
    name = "CREATE ARTICLE",
    fields(slug = tracing::field::Empty)
    skip_all,
)]
#[allow(unused_variables)]
pub async fn create_article(ctx: State<Arc<AppContext>>, id: UserID) {
    todo!()
}
