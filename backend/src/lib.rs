#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

mod config;
mod db;
mod http;
mod openapi;
mod telemetry;
mod utils;

use crate::http::catchers;
use crate::http::cors;
use crate::http::routes;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use rocket::figment::providers::Env;
use rocket::figment::providers::Serialized;
use rocket::{Build, Rocket};

/// Build `Rocket application.
///
/// Staging database and CORS elements is inspired by one of the official examples:
/// <https://github.com/rwf2/Rocket/blob/f9de1bf4671100b2f9c9bea6ce206fc4748ca999/examples/databases/src/main.rs>
pub fn construct_rocket(config: Option<Config>) -> Rocket<Build> {
    let config = match config {
        Some(overrides) => rocket::Config::figment()
            .merge(Env::raw())
            .merge(Serialized::globals(overrides)),
        None => rocket::Config::figment().merge(Env::raw()),
    };
    let custom: Config = config.extract().expect("config");
    let config = config.merge(("databases.main.url", custom.database_url));
    rocket::custom(config)
        .mount("/", routes![routes::healthz::health])
        .mount("/api", http::routes::users::routes())
        .register("/", catchers![catchers::unauthorized])
        .manage(EncodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
        .manage(DecodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
        .attach(db::stage(custom.migrate))
        .attach(cors::stage(custom.allowed_origins))
        .attach(openapi::stage())
}

// Making `Config` and `init_tracing` (alongside the `construct_rocket` builder)
// available for crate's consumers which is our `main.rs` binary - where we are
// initializing tracing, overriding configurations (if needed), then building
// and launching the app
pub use config::Config;
pub use telemetry::init_tracing;
