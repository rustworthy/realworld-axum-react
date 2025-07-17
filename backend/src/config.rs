use anyhow::Context as _;
use figment::{Figment, providers::Env};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub secret_key: String,
    pub migrate: bool,
    pub database_url: String,
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
