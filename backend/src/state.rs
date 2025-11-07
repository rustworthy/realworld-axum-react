use crate::services::mailer::ResendMailer;
use crate::services::moderator::Moderator;
use crate::{config::Config, services::captcha::Captcha};
use anyhow::Context;
use jsonwebtoken::{DecodingKey, EncodingKey};
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use url::Url;

pub(crate) struct AppContext {
    pub enc_key: EncodingKey,
    pub dec_key: DecodingKey,
    pub db: PgPool,
    pub mailer: ResendMailer,
    pub captcha: Captcha,
    pub moderator: Moderator,
    pub frontend_url: Url,
    pub skip_email_verification: bool,
    pub skip_captcha_verification: bool,
    pub skip_content_moderation: bool,
    pub skip_rate_limiting: bool,
}

impl AppContext {
    pub async fn try_build(config: &Config) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .connect(config.database_url.expose_secret())
            .await
            .context("Failed to connect to database")?;
        let secret = config.secret_key.expose_secret();
        let resend = ResendMailer::new(
            config.mailer_from.clone(),
            config.mailer_token.expose_secret(),
            config.mailer_endpoint.clone(),
            config.mailer_transport.clone(),
            None,
        );
        let captcha = Captcha::new(config.captcha_secret.clone(), None);
        let moderator = Moderator::new(
            config.openai_api_key.expose_secret().to_string(),
            config.openai_base_url.clone(),
        );

        let ctx = AppContext {
            enc_key: EncodingKey::from_base64_secret(secret)?,
            dec_key: DecodingKey::from_base64_secret(secret)?,
            db: pool.clone(),
            mailer: resend,
            captcha,
            moderator,
            frontend_url: config.frontend_url.clone(),
            skip_email_verification: config.skip_email_verification.unwrap_or_default(),
            skip_captcha_verification: config.skip_captcha_verification.unwrap_or_default(),
            skip_content_moderation: config.skip_content_moderation.unwrap_or_default(),
            skip_rate_limiting: config.skip_rate_limiting.unwrap_or_default(),
        };
        Ok(ctx)
    }
}
