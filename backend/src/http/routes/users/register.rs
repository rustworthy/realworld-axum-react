use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, ResultExt, Validation};
use crate::http::jwt::issue_token;
use crate::services::mailer::ResendMailer;
use crate::templates::{OTPEmailHtml, OTPEmailText};
use crate::utils::{gen_numeric_string, hash_password};
use anyhow::Context;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use chrono::Utc;
use resend_rs::types::EmailId;
use std::sync::Arc;
use std::time::Duration;
use tracing::Span;
use url::Url;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

const EMAIL_CONFIRMATION_TOKEN_LEN: usize = 8;
const EMAIL_CONFIRMATION_TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct Registration {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    #[schema(example = "rob.pike@gmail.com", format = "email")]
    #[validate(email(message = "invalid email format"))]
    email: String,

    /// User's name or nickname.
    ///
    /// This is - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(example = "rob.pike1984")]
    #[validate(length(min = 1, message = "username cannot be empty"))]
    username: String,

    /// User's password.
    ///
    /// There are currently no limitations on password strength.
    #[schema(min_length = 12, example = "Whoami@g00gle")]
    #[validate(length(min = 12, message = "password should be at least 12 characters long"))]
    password: String,
}

/// Register new user.
///
/// This will register new user in the system and also create
/// a JWT token, i.e. will immediate log them in.
#[utoipa::path(
    post,
    path = "",
    tags = ["Users"],
    responses(
        (status = 201, description = "User successfully created", body = UserPayload<User>),
        (status = 422, description = "Missing or invalid registration details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(/* authentication NOT required */),
)]
#[instrument(
    name = "REGISTER USER",
    fields(email_id = tracing::field::Empty)
    skip_all,
)]
pub(crate) async fn register_user(
    ctx: State<Arc<AppContext>>,
    input: Result<Json<UserPayload<Registration>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;

    // check email, username and password fields
    user.validate()?;

    let password_hash = hash_password(&user.password)?;

    let status = if ctx.skip_email_verification {
        "ACTIVE"
    } else {
        "EMAIL_CONFIRMATION_PENDING"
    };

    let user_uuid = sqlx::query_scalar!(
        r#"
            INSERT INTO "users" (email, username, password_hash, status) VALUES ($1, $2, $3, $4) RETURNING user_id 
        "#,
        &user.email,
        &user.username,
        &password_hash,
        status
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("users_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("users_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    if ctx.skip_email_verification {
        let jwt_string = issue_token(user_uuid, &ctx.enc_key).unwrap();

        let payload = UserPayload {
            user: User {
                email: user.email.clone(),
                token: jwt_string,
                username: user.username,
                bio: "".into(),
                image: None,
            },
        };

        return Ok(Json(payload));
    };

    let otp = gen_numeric_string(EMAIL_CONFIRMATION_TOKEN_LEN);
    let expires_at = Utc::now() + EMAIL_CONFIRMATION_TOKEN_TTL;

    sqlx::query!(
        r#"
            INSERT INTO "confirmation_tokens" (token, purpose, user_id, expires_at)
            VALUES ($1, 'EMAIL_CONFIRMATION', $2, $3)
        "#,
        &otp,
        &user_uuid,
        &expires_at
    )
    .execute(&ctx.db)
    .await?;

    let email_id =
        send_confirm_email_letter(&otp, &ctx.frontend_url, &user.email, &ctx.mailer).await?;
    Span::current().record("email_id", &*email_id);

    let jwt_string = issue_token(user_uuid, &ctx.enc_key).unwrap();

    let payload = UserPayload {
        user: User {
            email: user.email,
            token: jwt_string,
            username: user.username,
            bio: "".into(),
            image: None,
        },
    };
    Ok(Json(payload))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmailConfirmation {
    /// One-time password.
    ///
    /// An numeric code that has been sent to them upon registration.
    #[schema(min_length = 8, max_length = 8, example = "01234567")]
    otp: String,
}

/// Confirm email address.
#[utoipa::path(
    post,
    path = "/confirm-email",
    tags = ["Users"],
    responses(
        (status = 201, description = "User's email address confirmed", body = UserPayload<User>),
        (status = 422, description = "Missing or invalid email confirmation details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(/* authentication NOT required */),
)]
#[instrument(
    name = "CONFIRM EMAIL ADDRESS",
    fields(email_id = tracing::field::Empty)
    skip_all,
)]
pub(crate) async fn confirm_email(
    ctx: State<Arc<AppContext>>,
    input: Result<Json<UserPayload<EmailConfirmation>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;

    let user_id = sqlx::query_scalar!(
        r#"
            DELETE FROM "confirmation_tokens" 
            WHERE
                token = $1 and 
                purpose = 'EMAIL_CONFIRMATION' and
                expires_at > now()
            RETURNING user_id
        "#,
        &user.otp
    )
    .fetch_optional(&ctx.db)
    .await?
    .flatten();

    let user_id =
        user_id.ok_or_else(|| Error::unprocessable_entity([("otp", "Invalid or expired OTP")]))?;

    let user_row = sqlx::query!(
        r#"
            UPDATE "users"
            SET status = 'ACTIVE'
            WHERE user_id = $1
            RETURNING email, username
        "#,
        &user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    let jwt_string = issue_token(user_id, &ctx.enc_key).unwrap();

    let payload = UserPayload {
        user: User {
            email: user_row.email,
            token: jwt_string,
            username: user_row.username,
            bio: "".into(),
            image: None,
        },
    };

    Ok(Json(payload))
}

// ------------------------------ UTILITIES -----------------------------------
#[instrument(name = "EMAIL CONFIRMATION LETTER", skip(mailer, otp_code))]
async fn send_confirm_email_letter(
    otp_code: &str,
    app_url: &Url,
    to: &str,
    mailer: &ResendMailer,
) -> anyhow::Result<EmailId> {
    let html = OTPEmailHtml { otp_code, app_url }.to_string();
    let text = OTPEmailText { otp_code, app_url }.to_string();

    let email_id = mailer
        .send_email(to, "Let's confirm your email", &html, &text)
        .await
        .context("Failed to send OTP for email confirmation")?;

    Ok(email_id)
}
