use anyhow::Context as _;
use figment::{
    Figment,
    providers::{Env, Serialized},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub migrate: bool,
    pub database_url: String,
    pub allowed_origins: Vec<String>,
    pub secret_key: String,
    pub docs_ui_path: Option<String>,
    pub port: u16,
}

impl Config {
    pub fn try_build() -> anyhow::Result<Self> {
        let config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Env::raw())
            .extract()
            .context("Could not read configuration!")?;
        Ok(config)
    }
}
