use crate::http::errors::Error;
use crate::state::AppContext;
use axum::{Json, extract::State};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TagsList {
    /// List of tags.
    #[schema(
        example = json!(vec!["programming".to_string(), "language design".to_string()]),
        min_items = 0,
    )]
    tags: Vec<String>,
}

/// List tags.
///
/// Returns a list of tags registered in the system. The list will be sorted
/// by popularity (number of articles having this tag).
///
/// Note that if there are no articles in the system (or at least no articles
/// visible to the currently logged in user) the list can be empty.
///
/// No authentication required.
#[utoipa::path(
    get,
    path = "/",
    tags = ["Articles"],
    responses(
        (status = 200, description = "Tags list successfully retrieved", body = TagsList),
        (status = 500, description = "Internal server error."),
    ),
    security(
        (),
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "TAGS LIST", skip_all)]
pub async fn list_tags(ctx: State<Arc<AppContext>>) -> Result<Json<TagsList>, Error> {
    let tags = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count", UNNEST(tags) AS "tag!"
        FROM articles GROUP BY "tag!" ORDER BY "count" DESC;
    "#
    )
    .fetch_all(&ctx.db)
    .await?;
    Ok(Json(TagsList {
        tags: tags.into_iter().map(|row| row.tag).collect(),
    }))
}
