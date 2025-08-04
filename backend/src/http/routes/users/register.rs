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
use chrono::{DateTime, Utc};
use resend_rs::types::EmailId;
use std::sync::Arc;
use std::time::Duration;
use tracing::Span;
use url::Url;
use utoipa::ToSchema;
use uuid::Uuid;

const EMAIL_CONFIRMATION_TOKEN_LEN: usize = 8;
const EMAIL_CONFIRMATION_TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

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
    // @Dzmitry, we of course should not be just dropping user's password,
    // rather should verify it's not empty, hash, and store it in the database
    // we already got hashing function in the codebase, but we do not have
    // `user` table, neither sqlx query. It is the database engine that will
    // issue uuid and return it back to us.
    drop(user.password);

    if ctx.skip_email_verification {
        // @Dzmitry as if db engine returned this UUID to us;
        //  also - since we are skipping email verification here - let's
        //  make sure to set user status to "ACTIVE" (rather than, say,
        //  "EMAIL_CONFIRMATION_PENDING")
        let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();

        // let's issue a JWT for them to adhere to the Realworld project's spec;
        // this jwt could also be used as nonce in case we wanted to go sticter about
        // the email confirmation process: we could be expecting not only an OTP,
        // but also that nonce, meaining to confirm their email, they would need to use
        // the same browser app they've used to register (the client-side script then
        // would need to make sure to persist that token in their local storage)
        let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();

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
    }

    // create user with  "EMAIL_CONFIRMATION_PENDING" status
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();
    let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();

    // generate an OTP for them and persist it
    let otp = gen_alphanum_string(EMAIL_CONFIRMATION_TOKEN_LEN);
    let expires_at = Utc::now() + EMAIL_CONFIRMATION_TOKEN_TTL;

    sqlx::query!(
        r#"
            insert into "confirmation_tokens" (token, purpose, user_id, expires_at)
            values ($1, 'EMAIL_CONFIRMATION', null, $2)
        "#,
        &otp,
        &expires_at
    )
    .execute(&ctx.db)
    .await
    .map_err(|e| Error::Internal(e.to_string()))?;

    // now, encode it into an email and send them; also making sure to attach
    // the id of the sent email to the current span debugging (should it be needed)
    let email_id =
        send_confirm_email_letter(&otp, &ctx.frontend_url, &user.email, &ctx.mailer).await?;
    Span::current().record("email_id", &*email_id);

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
    /// An alphanumeAn alphanumeric code that has been sent to them upon registration.
    #[schema(min_length = 8, max_length = 8, example = "Aj23MpUI")]
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

    // @Dzmitry once the users table is ready, we will be able to fetch the user,
    // in the meantime let's mock the user the way we are doing in other endpoints
    {
        #[allow(unused)]
        #[derive(Debug)]
        struct Otp {
            id: i64,
            token: String,
            created_at: DateTime<Utc>,
            purpose: Option<String>,
            user_id: Option<Uuid>,
            expires_at: Option<DateTime<Utc>>,
        }
        let otp = sqlx::query_as!(
            Otp,
            r#"
                delete from "confirmation_tokens" 
                where
                    token = $1 and 
                    purpose = 'EMAIL_CONFIRMATION' and
                    expires_at > now()
                returning *
            "#,
            &user.otp
        )
        .fetch_optional(&ctx.db)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;
        dbg!(otp);
    }

    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();
    let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();
    let payload = UserPayload {
        user: User {
            email: "rob.pike@gmail.com".to_string(),
            token: jwt_string,
            username: "rob.pike".to_string(),
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
