use std::sync::Arc;

use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::jwt::issue_token;
use crate::utils::verify_password;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use url::Url;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub(crate) struct Login {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    #[schema(example = "rob.pike@gmail.com", format = "email")]
    #[validate(email(message = "invalid email format"))]
    email: String,

    /// User's password.
    #[schema(min_length = 12, examples("Whoami@g00gle",))]
    #[validate(length(min = 12, message = "password should be at least 12 characters long"))]
    password: String,
}

/// Log user in.
///
/// This will return user's details as well as a fresh JWT token.
#[utoipa::path(
    post,
    path = "/login",
    tags = ["Users"],
    responses(
        (status = 200, description = "User successfully logged in", body = UserPayload<User>),
        (status = 422, description = "Missing or invalid login details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(/* authentication NOT required */),
)]
#[instrument(name = "LOG USER IN", skip_all)]
pub(crate) async fn login(
    ctx: State<Arc<AppContext>>,
    login_details: Result<Json<UserPayload<Login>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = login_details?;

    // check email and password fields
    user.validate()?;

    let user_row = sqlx::query!(
        r#"
            SELECT user_id, username, email, bio, image, password_hash 
            FROM users 
            WHERE email = $1
        "#,
        &user.email
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::Unauthorized)?;

    let is_password_verified = verify_password(&user.password, &user_row.password_hash)?;

    if !is_password_verified {
        return Err(Error::Unauthorized);
    }

    let jwt_string = issue_token(user_row.user_id, &ctx.enc_key).unwrap();

    let image = user_row
        .image
        .as_deref()
        .map(|v| {
            Url::parse(v).map_err(|_| anyhow::anyhow!("Failed to parse store image path as URL"))
        })
        .transpose()?;

    let payload = UserPayload {
        user: User {
            email: user_row.email,
            token: jwt_string,
            username: user_row.username,
            bio: user_row.bio,
            image: image,
        },
    };

    Ok(Json(payload))
}
