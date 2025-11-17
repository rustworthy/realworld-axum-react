#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate utoipa_axum;
#[macro_use]
extern crate utoipa;

mod config;
mod http;
mod services;
mod state;
mod telemetry;
mod templates;
mod temporal;
mod utils;

use crate::http::layers::cors::cors_layer;
use crate::http::layers::rate::rate_limit_layer;
use crate::http::openapi;
use crate::http::routes;
use crate::state::AppContext;
use anyhow::Context;
use axum::Router;
use axum::http::header;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

// Making `Config` (and its components as needed) and `init_tracing` (alongside
// the `api` application builder available for crate's consumers which is our
// `main.rs` binary - where we are initializing tracing, overriding configurations
// (if needed), then building and launching the app
pub use config::{Config, MailerTransport};
pub use telemetry::init_tracing;

static OPENAPI_JSON: OnceLock<&'static str> = OnceLock::new();

pub async fn api(config: Config) -> anyhow::Result<Router> {
    // ------------------------- PREPARE CONTEXT -------------------------------
    let ctx = Arc::new(AppContext::try_build(&config).await?);

    // ------------------------- PREPARE AXUM APP ------------------------------
    let (app, docs) = OpenApiRouter::with_openapi(openapi::RootApiDoc::openapi())
        .route("/healthz", get(routes::healthz::health))
        .with_state(Arc::clone(&ctx))
        .nest("/api", routes::users::router(Arc::clone(&ctx)))
        .nest("/api", routes::articles::router(Arc::clone(&ctx)))
        .layer(rate_limit_layer(ctx.redis.clone(), ctx.skip_rate_limiting)?)
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10))
        .layer(cors_layer(config.allowed_origins))
        .layer(SetSensitiveHeadersLayer::new([header::AUTHORIZATION]))
        .layer(CatchPanicLayer::new())
        .split_for_parts();

    // ------------------------ ATTACH DOCUMENTATION ---------------------------
    let oai = OPENAPI_JSON.get_or_init(|| docs.to_json().expect("valid JSON").leak());
    #[allow(unused_mut)]
    let mut app = app.merge(
        Router::new()
            .route(
                "/openapi.json",
                get(|| async {
                    (
                        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
                        *oai,
                    )
                }),
            )
            .fallback_service(ServeDir::new("./static")),
    );

    // -------------------------- ATTACH DEBUG ROUTES --------------------------
    #[cfg(debug_assertions)]
    {
        app = app.merge(
            Router::new()
                .route(
                    "/dev/preview/otp_email",
                    get(routes::dev::preview_otp_email),
                )
                .with_state(Arc::clone(&ctx)),
        );
    }

    // -------------------------- RUN MIGRATIONS -------------------------------
    if config.migrate.unwrap_or_default() {
        info!("Applying database migrations");
        sqlx::migrate!()
            .run(&ctx.db)
            .await
            .context("failed to run migrations")?;
    }

    if let Some(url) = &config.temporal_url {
        let mut client = temporal::init_client(url.to_owned()).await?;
        let _resp = temporal::create_maintenance_schedule(&mut client).await?;
    }

    Ok(app)
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let addr = SocketAddr::from((config.ip, config.port));
    let listener = TcpListener::bind(addr).await?;
    let app = api(config).await?;
    info!("Launching application at {:?}", &addr);
    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

/// Graceful shutdown signal.
///
/// Source: <https://github.com/davidpdrsn/realworld-axum-sqlx/blob/d03a2885b661c8466de24c507099e0e2d66b55bd/src/http/mod.rs>
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
