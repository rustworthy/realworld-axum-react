use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::jwt::issue_token;
use crate::services::mailer::ResendMailer;
use crate::templates::{OTPEmailHtml, OTPEmailText};
use crate::utils::gen_alphanum_string;
use anyhow::Context;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct Registration {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    email: String,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    username: String,

    /// User's password.
    ///
    /// There are currently no limitations on password strength.
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
#[instrument(name = "REGISTER USER", skip_all)]
pub(crate) async fn register_user(
    ctx: State<Arc<AppContext>>,
    input: Result<Json<UserPayload<Registration>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;
    // @Dzmitry, we of course should not be just dropping user's password,
    // rather should verify it's not empty, hash, and store it in the database
    // we already got hashing function in the codebase, but we do not have
    // `user` table, neither sqlx query. It is the database engine that will
    // issue uuid and return it back to us.
    drop(user.password);

    // @Dzmitry as if db engine returned this UUID to us
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();

    let otp = gen_alphanum_string(8);
    send_confirm_email_letter(&otp, &ctx.frontend_url, &user.email, &ctx.mailer).await?;

    // @Dzmitry and we issued a token for the newly created user
    let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();

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

#[instrument(name = "EMAIL CONFIRMATION LETTER", skip(mailer, otp_code))]
async fn send_confirm_email_letter(
    otp_code: &str,
    app_url: &Url,
    to: &str,
    mailer: &ResendMailer,
) -> anyhow::Result<()> {
    let html = OTPEmailHtml { otp_code, app_url }.to_string();
    let text = OTPEmailText { otp_code, app_url }.to_string();
    mailer
        .send_email(to, "Let's confirm your email", &html, &text)
        .await
        .context("Failed to send OTP for email confirmation")
}
