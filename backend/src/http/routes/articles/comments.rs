use super::Author;
use crate::http::errors::{Error, ResultExt as _, Validation};
use crate::http::extractors::UserID;
use crate::http::routes::users::utils::parse_image_url;
use crate::state::AppContext;
use axum::extract::{Json, Path, State};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

/// Container for comment creation endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct CommentPayload<U> {
    comment: U,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub(crate) struct CommentCreate {
    /// Comment's text.
    #[schema(
        examples("Memory safety matters, but it's all about ecosystem imho",),
        min_length = 1,
        max_length = 500
    )]
    #[validate(length(
        min = 1,
        max = 500,
        message = "comment should be at least 1 and at max 500 characters long"
    ))]
    body: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Comment {
    /// Comment's unique identifier.
    id: Uuid,

    /// When this comment was created.
    created_at: DateTime<Utc>,

    /// When this comment was last update.
    updated_at: DateTime<Utc>,

    /// Comment's text.
    body: String,

    /// Details of the comment's author
    author: Author,
}

/// Add comment to article.
///
/// Authentication required.
#[utoipa::path(
    post,
    path = "/{slug}/comments",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path,
            format = "slug",
            description = "Article's slug identifier.",
            example = "why-memory-safety-matters"
        ),
    ),
    responses(
        (status = 200, description = "Comment successfully created", body = CommentPayload<Comment>),
        (status = 401, description = "Token missing or invalid."),
        (status = 404, description = "Article not found"),
        (status = 415, description = "Method not allow / Content-Type is incorrect"),
        (status = 422, description = "Missing or invalid comment attributes", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "CREATE COMMENT", skip(ctx))]
#[allow(unused)]
pub async fn create_comment(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: UserID,
    Json(CommentPayload { comment }): Json<CommentPayload<CommentCreate>>,
) -> Result<Json<CommentPayload<Comment>>, Error> {
    comment.validate()?;

    let data = sqlx::query!(
        r#"
        WITH comment AS (
            INSERT INTO comments (article_id, user_id, body)
            SELECT article_id, $2, $3 FROM articles WHERE slug = $1
            RETURNING comment_id, created_at, updated_at
        )
        SELECT
            comment.comment_id AS comment_id,
            comment.created_at AS comment_created_at,
            comment.updated_at AS comment_updated_at,
            comment_author.bio AS comment_author_bio,
            comment_author.username AS comment_author_username,
            comment_author.image AS comment_author_image
        FROM comment JOIN users comment_author ON user_id = $2
        "#,
        &slug,
        *uid,
        &comment.body
    )
    .fetch_optional(&ctx.db)
    .await
    .on_constraint("comments_user_id_fkey", |_| Error::Unauthorized)?
    .ok_or(Error::NotFound)?;

    let payload = CommentPayload {
        comment: Comment {
            id: data.comment_id,
            created_at: data.comment_created_at,
            updated_at: data.comment_updated_at.unwrap_or(data.comment_created_at),
            body: comment.body,
            author: Author {
                username: data.comment_author_username,
                bio: data.comment_author_bio,
                image: parse_image_url(data.comment_author_image.as_deref())?,
                // reminder: we do not allow users to follow themselves, so
                // the comment's author cannot follow the calling user since
                // this is the same actor in this case;
                following: false,
            },
        },
    };

    Ok(Json(payload))
}
