use anyhow::Context as _;
use figment::{Figment, providers::Env};
use secrecy::SecretString;
use std::net::IpAddr;
use url::Url;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MailerTransport {
    #[default]
    Http,
    Stdout,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub secret_key: SecretString,
    pub database_url: SecretString,
    #[serde(default)]
    pub mailer_transport: MailerTransport,
    pub mailer_token: SecretString,
    pub mailer_endpoint: Url,
    pub mailer_from: String,
    pub frontend_url: Url,
    pub migrate: bool,
    pub allowed_origins: Vec<String>,
    pub ip: IpAddr,
    pub port: u16,
    pub docs_ui_path: Option<String>,

    /// Skip email verification logic.
    ///
    /// This is something we only need to satisfy Realdworld project's
    /// end-to-end test suite: the spec allows to create an account as
    /// long as the email is unique in the system, but it does not check
    /// whether it exists and belongs to the user who is trying to register.
    pub skip_email_verification: Option<bool>,
}

impl Config {
    pub fn try_build() -> anyhow::Result<Self> {
        let config: Config = Figment::new()
            .merge(Env::raw())
            .extract()
            .context("Could not read configuration!")?;
        Ok(config)
    }
}
