#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate utoipa_axum;

mod config;
mod http;
mod services;
mod state;
mod telemetry;
mod utils;

use crate::http::cors;
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
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

// Making `Config` and `init_tracing` (alongside the `api` application builder
// available for crate's consumers which is our `main.rs` binary - where we are
// initializing tracing, overriding configurations (if needed), then building
// and launching the app
pub use config::Config;
pub use telemetry::init_tracing;

static OPENAPI_JSON: OnceLock<&'static str> = OnceLock::new();

/// Scalar initial html.
///
/// Using [`solarized`](https://guides.scalar.com/scalar/scalar-api-references/themes)
/// theme and ["suppressing"](https://stackoverflow.com/a/13416784) browser's favicon
/// not found error.
///
/// If there is a need for a customized icon or there is a requirement to
/// serve all the assets (scripts, fonts, images) from our server, this html
/// can be placed to a directory like `docs` or `templates` (especially if a template
/// engine is used) and co-located with the vendors' assets (including the scalar build
/// that we can download from the CDN ahead of time), which then can be served
/// with [`ServeDir`](https://docs.rs/tower-http/latest/tower_http/services/struct.ServeDir.html)
static SCALAR_HTML: &str = r#"
    <!doctype html>
    <html>
    <head>
        <title>Realworld Axum React | API Docs</title>
        <meta charset="utf-8"/>
        <link rel="icon" href="data:image/png;base64,iVBORw0KGgo=">
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
    </head>
    <body>
        <noscript>
            Scalar requires Javascript to function. Please enable it to browse the documentation.
        </noscript>
        <script 
            id="api-reference" 
            data-configuration='{"theme": "solarized"}' 
            data-url="openapi.json" 
        >
        </script>
        <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
    </body>
    </html>
"#;

pub async fn api(config: Config) -> anyhow::Result<Router> {
    // ------------------------- PREPARE CONTEXT -------------------------------
    let ctx = Arc::new(AppContext::try_build(&config).await?);

    // ------------------------- PREPARE AXUM APP ------------------------------
    let (app, docs) = OpenApiRouter::with_openapi(openapi::RootApiDoc::openapi())
        .route("/healthz", get(routes::healthz::health))
        .with_state(Arc::clone(&ctx))
        .nest("/api", routes::users::router(Arc::clone(&ctx)))
        .layer(cors::layer(config.allowed_origins))
        .split_for_parts();

    // ----------------------- PREPARE DOCUMENTATION ---------------------------
    let oai = OPENAPI_JSON.get_or_init(|| docs.to_json().unwrap().leak());
    let app = app.merge(
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
            .route(
                &config.docs_ui_path.unwrap_or("/".to_string()),
                get(|| async { ([(header::CONTENT_TYPE, "text/html")], SCALAR_HTML) }),
            ),
    );

    // -------------------------- RUN MIGRATIONS -------------------------------
    sqlx::migrate!()
        .run(&ctx.db)
        .await
        .context("failed to run migrations")?;

    Ok(app)
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let addr = SocketAddr::from((config.ip, config.port));
    let listener = TcpListener::bind(addr).await?;
    let app = api(config).await?;
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
