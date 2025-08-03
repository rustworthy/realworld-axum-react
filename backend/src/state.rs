use crate::config::Config;
use crate::services::mailer::ResendMailer;
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
    pub frontend_url: Url,
    pub skip_email_verification: bool,
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
        let ctx = AppContext {
            enc_key: EncodingKey::from_base64_secret(secret)?,
            dec_key: DecodingKey::from_base64_secret(secret)?,
            db: pool.clone(),
            mailer: resend,
            frontend_url: config.frontend_url.clone(),
            skip_email_verification: config.skip_email_verification.unwrap_or_default(),
        };
        Ok(ctx)
    }
}
