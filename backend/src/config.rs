use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub migrate: bool,
    pub database_url: String,
    pub allowed_origins: Vec<String>,
    pub secret_key: String,
    pub docs_ui_path: Option<String>,
    pub port: u16,
}
