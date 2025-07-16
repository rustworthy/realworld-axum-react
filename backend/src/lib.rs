#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;

mod config;
mod db;
mod http;
mod openapi;
mod telemetry;
mod utils;

use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::http::cors;
use crate::http::routes;
use axum::Router;
use axum::routing::get;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use rocket::figment::providers::Env;
use rocket::figment::providers::Serialized;

// Making `Config` and `init_tracing` (alongside the `api` application builder)
// available for crate's consumers which is our `main.rs` binary - where we are
// initializing tracing, overriding configurations (if needed), then building
// and launching the app
pub use config::Config;
pub use telemetry::init_tracing;

#[derive(Clone)]
pub(crate) struct AppContext {
    pub enc_key: Arc<EncodingKey>,
    pub dec_key: Arc<DecodingKey>,
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let ctx = AppContext {
        enc_key: Arc::new(EncodingKey::from_base64_secret(&config.secret_key)?),
        dec_key: Arc::new(DecodingKey::from_base64_secret(&config.secret_key)?),
    };
    //let config = match config {
    //    Some(overrides) => rocket::Config::figment()
    //        .merge(Env::raw())
    //            .merge(Serialized::globals(overrides)),
    //    None => rocket::Config::figment().merge(Env::raw()),
    //};
    // let custom: Config = config.extract().expect("config");
    // let config = config.merge(("databases.main.url", custom.database_url));
    let app = Router::new()
        .route("/healthz", get(routes::healthz::health))
        .with_state(ctx)
        .layer(cors::layer(config.allowed_origins));
    //  .mount("/api", http::routes::users::routes())
    //  .register("/", catchers![catchers::unauthorized])
    //  .manage(EncodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
    //  .manage(DecodingKey::from_base64_secret(&custom.secret_key).expect("valid base64"))
    //  .attach(db::stage(custom.migrate))
    //  .attach(cors::stage(custom.allowed_origins))
    //  .attach(openapi::stage(custom.docs_ui_path))

    let ipv4: Ipv4Addr = "127.0.0.1".parse()?;
    let addr = SocketAddr::from((ipv4, 8000));
    let listener = TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// source: https://github.com/davidpdrsn/realworld-axum-sqlx/blob/d03a2885b661c8466de24c507099e0e2d66b55bd/src/http/mod.rs
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
