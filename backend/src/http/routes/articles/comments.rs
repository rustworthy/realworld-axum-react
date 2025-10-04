use super::Author;
use crate::http::errors::{Error, ResultExt as _, Validation};
use crate::http::extractors::{MaybeUserID, UserID};
use crate::http::routes::users::utils::parse_image_url;
use crate::state::AppContext;
use axum::extract::{Json, Path, State};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

// ----------------------------- CREATE COMMENT -------------------------------
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

// -------------------------------- LIST COMMENTS -----------------------------

/// Contrainer for list comments endpoint.
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct CommentsList {
    /// List of comments.
    comments: Vec<Comment>,
}

/// List comments to article.
///
/// Authentication is optional.
#[utoipa::path(
    get,
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
        (status = 200, description = "Comments list successfully retrieved", body = CommentsList),
        (status = 401, description = "Token missing or invalid (in case authenicated access has been used)"),
        (status = 500, description = "Internal server error."),
    ),
    security(
        (),
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "LIST COMMENTS", skip(ctx))]
pub async fn list_comments(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: MaybeUserID,
) -> Result<Json<CommentsList>, Error> {
    let article_id = sqlx::query_scalar!(r"SELECT article_id FROM articles WHERE slug = $1", &slug)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let comments = sqlx::query!(
        r#"
        SELECT
            comment.comment_id AS comment_id,
            comment.created_at AS comment_created_at,
            comment.updated_at AS comment_updated_at,
            comment.body AS comment_body,
            comment_author.bio AS comment_author_bio,
            comment_author.username AS comment_author_username,
            comment_author.image AS comment_author_image,
            (
                $1::UUID IS NOT NULL AND EXISTS
                    (
                        SELECT 1 FROM follows
                        WHERE followed_user_id = comment_author.user_id
                        AND following_user_id = $1
                    )
            ) AS "comment_author_following!"
        FROM comments comment JOIN users comment_author USING (user_id)
        WHERE comment.article_id = $2
        "#,
        uid.0.as_deref(),
        article_id,
    )
    .fetch_all(&ctx.db)
    .await?;
    let payload = CommentsList {
        comments: comments
            .into_iter()
            .map(|data| {
                let comment = Comment {
                    id: data.comment_id,
                    created_at: data.comment_created_at,
                    updated_at: data.comment_updated_at.unwrap_or(data.comment_created_at),
                    body: data.comment_body,
                    author: Author {
                        username: data.comment_author_username,
                        bio: data.comment_author_bio,
                        image: parse_image_url(data.comment_author_image.as_deref())?,
                        following: data.comment_author_following,
                    },
                };
                Ok(comment)
            })
            .collect::<Result<_, Error>>()?,
    };
    Ok(Json(payload))
}

// ------------------------------ DELETE COMMENT -------------------------------

/// Delete comment.
///
/// Authentication required.
#[utoipa::path(
    delete,
    path = "/{slug}/comments/{comment_id}",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path,
            format = "slug",
            description = "Article's slug identifier.",
            example = "why-memory-safety-matters",
        ),
        (
            "comment_id" = String, Path,
            format = Uuid,
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),

    ),
    responses(
        (status = 204, description = "Comment deleted"),
        (status = 401, description = "Token missing or invalid"),
        (status = 403, description = "User does not have permissions to delete this comment"),
        (status = 404, description = "Article or comment not found"),
        (status = 422, description = "Malformed comment_id in path", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "DELETE COMMENT", skip(ctx))]
pub async fn delete_comment(
    ctx: State<Arc<AppContext>>,
    Path((slug, comment_id)): Path<(String, String)>,
    uid: UserID,
) -> Result<StatusCode, Error> {
    let comment_id = Uuid::parse_str(&comment_id)
        .map_err(|_| Error::unprocessable_entity([("path", "comment_id is not a valid UUID")]))?;

    let res = sqlx::query!(
        r#"
        WITH
            comment_deleted AS (
                DELETE FROM comments
                WHERE comment_id = $1 AND user_id = $2
                RETURNING 1
            ),
            comment_existed AS (
                SELECT 1 FROM comments WHERE comment_id = $1::TEXT::UUID
            ),
            article_exists AS (
                SELECT 1 FROM articles WHERE slug = $3
            )
        SELECT
            EXISTS(SELECT 1 FROM comment_deleted) AS "comment_deleted!",
            EXISTS(SELECT 1 FROM comment_existed) AS "comment_existed!",
            EXISTS(SELECT 1 FROM article_exists) AS "article_exists!"
        "#,
        &comment_id,
        *uid,
        &slug,
    )
    .fetch_one(&ctx.db)
    .await?;

    if res.comment_deleted {
        return Ok(StatusCode::NO_CONTENT);
    }

    if !res.article_exists || !res.comment_existed {
        return Err(Error::NotFound);
    }

    Err(Error::Forbidden)
}
