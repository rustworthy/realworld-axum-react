use anyhow::Context as _;
use figment::{Figment, providers::Env};
use secrecy::SecretString;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub secret_key: SecretString,
    pub database_url: SecretString,

    pub migrate: bool,
    pub allowed_origins: Vec<String>,
    pub ip: IpAddr,
    pub port: u16,
    pub docs_ui_path: Option<String>,
}

impl Config {
    pub fn try_build() -> anyhow::Result<Self> {
        let config = Figment::new()
            .merge(Env::raw())
            .extract()
            .context("Could not read configuration!")?;
        Ok(config)
    }
}
