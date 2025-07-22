use crate::config::Config;
use crate::services::mailer::{Mailer, ResendMailer, StdoutMailer};
use anyhow::{Context, anyhow};
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

        // if API key to access a mailer service is specified, we _also_ expect
        // "from" address to be configured, while the mailer's endpoint we default
        // to the well-known Resend's endpoint (`resend_rs` does it under the hood)
        let ctx = match config.mailer_token.as_ref() {
            Some(token) => {
                let sender_addr = config
                    .mailer_from
                    .as_ref()
                    .ok_or_else(|| anyhow!("from address is not configured for mail"))?
                    .to_owned();
                let mailer = ResendMailer::new(
                    sender_addr,
                    token.expose_secret(),
                    config.mailer_endpoint.clone(),
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
                let mailer = StdoutMailer::new(
                    config
                        .mailer_from
                        .as_ref()
                        .unwrap_or(&"onboarding@resend.dev".to_string())
                        .to_owned(),
                );
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
