use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub migrate: bool,
    pub database_url: String,
    pub allowed_origins: Option<Vec<String>>,
}
