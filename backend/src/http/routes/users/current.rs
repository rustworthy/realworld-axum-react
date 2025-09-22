use super::utils;
use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, ResultExt, Validation};
use crate::http::extractors::UserID;
use crate::http::jwt::issue_token;
use crate::utils::hash_password;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

/// Read current user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    get,
    path = "",
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Token missing or invalid."),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "GET CURRENT USER", skip(ctx))]
#[axum::debug_handler]
pub(crate) async fn read_current_user(
    ctx: State<Arc<AppContext>>,
    id: UserID,
) -> Result<Json<UserPayload<User>>, Error> {
    let jwt_string = issue_token(id.0, &ctx.enc_key).unwrap();
    let payload = UserPayload {
        user: User {
            email: "pavel@mikhalkevich.com".into(),
            token: jwt_string,
            username: "pavel.mikhalkevich".into(),
            bio: "".into(),
            image: None,
        },
    };
    Ok(Json(payload))
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UserUpdate {
    /// User's email, e.g. `rob.pike@gmail.com`.
    #[schema(nullable = false, example = "rob.pike@gmail.com", format = "email")]
    #[validate(email(message = "invalid email format"))]
    email: Option<String>,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(nullable = false, example = "rob.pike1984")]
    #[validate(length(min = 1, message = "username cannot be empty"))]
    username: Option<String>,

    /// User's biography.
    ///
    /// Note that Empty string will override the existing biography.
    #[schema(
        nullable = false,
        example = "In 2007, while working at Google, I designed Go together with Robert Griesemer and Ken Thompson"
    )]
    bio: Option<String>,

    /// New password.
    #[schema(nullable = false, min_length = 12, example = "Whoami@g00gle")]
    #[validate(length(min = 12, message = "password should be at least 12 characters long"))]
    password: Option<String>,

    /// New image URL.
    ///
    /// Specifying `null` means removing the image altogether.
    #[serde(default, with = "::serde_with::rust::double_option")]
    image: Option<Option<Url>>,
}

/// Update current user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    put,
    path = "",
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Authentication required."),
        (status = 422, description = "Missing or invalid registration details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "UPDATE CURRENT USER", skip(ctx))]
pub(crate) async fn update_current_user(
    ctx: State<Arc<AppContext>>,
    id: UserID,
    input: Result<Json<UserPayload<UserUpdate>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;

    user.validate()?;

    let password_hash = if let Some(password) = user.password {
        Some(hash_password(password)?)
    } else {
        None
    };

    let updated_image = match &user.image {
        None => None,
        Some(None) => Some(""),
        Some(Some(url)) => Some(url.as_str()),
    };

    let updated_user = sqlx::query!(
        r#"
            UPDATE "users"
            SET email = coalesce($1, "users".email),
                username = coalesce($2, "users".username),
                bio = coalesce($3, "users".bio),
                password_hash = coalesce($4, "users".password_hash),
                image = coalesce($5, "users".image)
            WHERE user_id = $6
            returning email, username, bio, image
        "#,
        user.email,
        user.username,
        user.bio,
        password_hash,
        updated_image,
        id.0
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    let jwt_string = issue_token(id.0, &ctx.enc_key).unwrap();

    let payload = UserPayload {
        user: User {
            email: updated_user.email,
            token: jwt_string,
            username: updated_user.username,
            bio: updated_user.bio,
            image: utils::parse_image_url(updated_user.image.as_deref())?,
        },
    };

    Ok(Json(payload))
}
