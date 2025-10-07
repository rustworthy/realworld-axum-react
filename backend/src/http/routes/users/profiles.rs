use std::sync::Arc;

use super::{UserProfile, UserProfilePayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::extractors::{MaybeUserID, UserID};
use crate::http::routes::users::utils::parse_image_url;
use axum::extract::{Json, Path, State};

/// Get user profile.
///
/// This will return user's profile. Can be retrieved with and without token.
#[utoipa::path(
    get,
    path = "/{username}",
    tags = ["Profiles"],
    params(
        (
            "username" = String, Path,
        ),
    ),
    responses(
        (status = 200, description = "User profile successfully retrieved", body = UserProfilePayload<UserProfile>),
        (status = 401, description = "Unauthorized", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(
        (),
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "GET USER'S PROFILE", skip(ctx))]
pub(crate) async fn profile(
    ctx: State<Arc<AppContext>>,
    Path(username): Path<String>,
    uid: MaybeUserID,
) -> Result<Json<UserProfilePayload<UserProfile>>, Error> {
    let user_profile = sqlx::query!(
        r#"
        SELECT
            username,
            bio,
            image,
            user_id,
            (
                $1::UUID IS NOT NULL AND EXISTS
                    (
                        SELECT 1 FROM follows
                        WHERE followed_user_id = user_id
                        AND following_user_id = $1
                    )
            ) AS "following!"
        FROM users
        WHERE username = $2
        "#,
        uid.0.as_deref(),
        username,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let payload = UserProfilePayload {
        profile: UserProfile {
            username: user_profile.username,
            bio: user_profile.bio,
            image: parse_image_url(user_profile.image.as_deref())?,
            following: user_profile.following,
        },
    };

    Ok(Json(payload))
}

/// Follow user profile.
///
/// This will follow user and return user's profile.
#[utoipa::path(
    post,
    path = "/{username}/follow",
    tags = ["Profiles"],
    params(
        (
            "username" = String, Path
        ),
    ),
    responses(
        (status = 200, description = "User successfully started follow current user's profile", body = UserProfilePayload<UserProfile>),
        (status = 401, description = "Unauthorized", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "FOLLOW USER PROFILE", skip(ctx))]
pub(crate) async fn follow_profile(
    ctx: State<Arc<AppContext>>,
    Path(username): Path<String>,
    uid: UserID,
) -> Result<Json<UserProfilePayload<UserProfile>>, Error> {
    let target_user = sqlx::query!(
        r#"
            WITH target AS (
                SELECT user_id, username, bio, image
                FROM users
                WHERE username = $1
            ),
            inserted AS (
                INSERT INTO follows (following_user_id, followed_user_id, updated_at)
                SELECT $2, target.user_id, NOW()
                FROM target
                ON CONFLICT DO NOTHING
            )
            SELECT
                target.username,
                target.bio,
                target.image,
                target.user_id,
                TRUE AS "user_following!"
            FROM target
            "#,
        username,
        uid.0
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let payload = UserProfilePayload {
        profile: UserProfile {
            username: target_user.username,
            bio: target_user.bio,
            image: parse_image_url(target_user.image.as_deref())?,
            following: target_user.user_following,
        },
    };

    Ok(Json(payload))
}

/// Unfollow user profile.
///
/// This will unfollow user and return user's profile.
#[utoipa::path(
    delete,
    path = "/{username}/follow",
    tags = ["Profiles"],
    params(
        (
            "username" = String, Path
        ),
    ),
    responses(
        (status = 200, description = "User successfully unfollow from current user's profile", body = UserProfilePayload<UserProfile>),
        (status = 401, description = "Unauthorized", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "UNFOLLOW USER PROFILE", skip(ctx))]
pub(crate) async fn unfollow_profile(
    ctx: State<Arc<AppContext>>,
    Path(username): Path<String>,
    uid: UserID,
) -> Result<Json<UserProfilePayload<UserProfile>>, Error> {
    let target_user = sqlx::query!(
        r#"
            WITH target AS (
                SELECT user_id, username, bio, image
                FROM users
                WHERE username = $1
            ),
            deleted AS (
                DELETE FROM follows WHERE following_user_id = $2 AND followed_user_id = (SELECT user_id FROM target)
            )
            SELECT
                target.username,
                target.bio,
                target.image,
                target.user_id,
                FALSE AS "user_following!"
            FROM target
            "#,
        username,
        uid.0,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let payload = UserProfilePayload {
        profile: UserProfile {
            username: target_user.username,
            bio: target_user.bio,
            image: parse_image_url(target_user.image.as_deref())?,
            following: target_user.user_following,
        },
    };

    Ok(Json(payload))
}
