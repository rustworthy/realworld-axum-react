use crate::config::Config;
use crate::services::mailer::{Mailer, ResendMailer, StdoutMailer};
use anyhow::Context;
use jsonwebtoken::{DecodingKey, EncodingKey};
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub(crate) struct AppContext {
    pub enc_key: EncodingKey,
    pub dec_key: DecodingKey,
    pub db: PgPool,
    #[allow(unused)]
    pub mailer: Box<dyn Mailer + Send + Sync + 'static>,
}

impl AppContext {
    pub async fn try_build(config: &Config) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .connect(config.database_url.expose_secret())
            .await
            .context("Failed to connect to database")?;
        let secret = config.secret_key.expose_secret();

        let ctx = match config.mailer_token.as_ref() {
            Some(token) => {
                let mailer = ResendMailer::new(
                    "onboarding@resend.dev".to_string(),
                    token.expose_secret(),
                    None,
                );
                AppContext {
                    enc_key: EncodingKey::from_base64_secret(secret)?,
                    dec_key: DecodingKey::from_base64_secret(secret)?,
                    db: pool.clone(),
                    mailer: Box::new(mailer),
                }
            }
            None if cfg!(debug_assertions) => {
                let mailer = StdoutMailer::new("onboarding@resend.dev".to_string());
                AppContext {
                    enc_key: EncodingKey::from_base64_secret(secret)?,
                    dec_key: DecodingKey::from_base64_secret(secret)?,
                    db: pool.clone(),
                    mailer: Box::new(mailer),
                }
            }
            _ => anyhow::bail!("mailer token is not configured"),
        };

        Ok(ctx)
    }
}
