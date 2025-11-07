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
mod utils;

use crate::http::cors;
use crate::http::openapi;
use crate::http::routes;
use crate::state::AppContext;
use anyhow::Context;
use axum::Router;
use axum::http::{Method, header};
use axum::routing::get;
use secrecy::ExposeSecret;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::services::ServeDir;
use tower_redis_cell::ProvideRuleError;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

// Making `Config` (and its components as needed) and `init_tracing` (alongside
// the `api` application builder available for crate's consumers which is our
// `main.rs` binary - where we are initializing tracing, overriding configurations
// (if needed), then building and launching the app
pub use config::{Config, MailerTransport};
pub use telemetry::init_tracing;

static OPENAPI_JSON: OnceLock<&'static str> = OnceLock::new();

use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::AppendHeaders;
use axum::response::IntoResponse as _;
use deadpool_redis::{Config as DeadpoolConfig, Runtime};
use tower_redis_cell::Error as RateLimitError;
use tower_redis_cell::RateLimitConfig;
use tower_redis_cell::deadpool::RateLimitLayer;
use tower_redis_cell::redis_cell::{Key, Policy};
use tower_redis_cell::{ProvideRule, ProvideRuleResult, Rule};

const BASIC_POLICY: Policy = Policy::from_tokens_per_minute(120)
    .max_burst(120)
    .name("basic");
const STRICT_POLICY: Policy = Policy::from_tokens_per_day(5).max_burst(5).name("strict");
const VERY_STRICT_POLICY: Policy = Policy::from_tokens_per_day(1)
    .max_burst(1)
    .name("very_strict");

#[derive(Clone, Debug, Default)]
struct RuleProvider {
    skip_rate_limiting: bool,
}

impl<T> ProvideRule<Request<T>> for RuleProvider {
    fn provide<'a>(&self, req: &'a Request<T>) -> ProvideRuleResult<'a> {
        // we want to accomodate for the end-to-end test suites
        if self.skip_rate_limiting {
            return Ok(None);
        }

        let ip = if cfg!(debug_assertions) {
            "localhost"
        } else {
            req.headers()
                .get("x-forwarded-for")
                .ok_or("'x-forwarded-for' header is missing")?
                .to_str()
                .map_err(|e| ProvideRuleError::default().detail(e.to_string()))?
        };

        let (path, method) = (req.uri().path(), req.method());

        // writing and updating articles is a fairly expensive operation due to
        // content moderation and so we are applying a stricter policy
        if path.contains("/articles") && (method == Method::POST || method == Method::PUT) {
            let key = Key::triple(ip, path, method.as_str());
            let rule = Rule::new(key, STRICT_POLICY).resource("articles::create");
            return Ok(Some(rule));
        }

        // we allow them to login quite a few times a day from the same
        // IP address ...
        if path.ends_with("/users/login") {
            let key = Key::triple(ip, path, method.as_str());
            return Ok(Some(Rule::new(key, STRICT_POLICY)));
        }

        // ... but  we want to impose a very strict limit on the number of
        // accounts they can create using the same IP address
        if path.ends_with("/users/confirm-email") || path.ends_with("/users") {
            let key = Key::triple(ip, path, method.as_str());
            return Ok(Some(Rule::new(key, VERY_STRICT_POLICY)));
        }
        let key = Key::pair(ip, path);
        dbg!(&key);
        Ok(Some(Rule::new(key, BASIC_POLICY)))
    }
}

pub async fn api(config: Config) -> anyhow::Result<Router> {
    // ------------------------- PREPARE CONTEXT -------------------------------
    let ctx = Arc::new(AppContext::try_build(&config).await?);

    let rate_limit_config = RateLimitConfig::new(
        RuleProvider {
            skip_rate_limiting: ctx.skip_rate_limiting,
        },
        |err, _req| match err {
            RateLimitError::ProvideRule(err) => {
                tracing::warn!(
                    key = ?err.key,
                    detail = err.detail.as_deref(),
                    "failed to define rule for request"
                );
                (StatusCode::UNAUTHORIZED, err.to_string()).into_response()
            }
            RateLimitError::RateLimit(err) => {
                tracing::warn!(
                    key = %err.rule.key,
                    policy = err.rule.policy.name,
                    "request throttled"
                );
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    AppendHeaders([(header::RETRY_AFTER, err.details.retry_after)]),
                    Body::from("too many requests"),
                )
                    .into_response()
            }
            err => {
                tracing::error!(err = %err, "unexpected error");
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        },
    );

    let cfg = DeadpoolConfig::from_url(config.redis_url.expose_secret());
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .context("failed to create pool")?;
    let rate_limit_layer = RateLimitLayer::new(rate_limit_config, pool);

    // ------------------------- PREPARE AXUM APP ------------------------------
    let (app, docs) = OpenApiRouter::with_openapi(openapi::RootApiDoc::openapi())
        .route("/healthz", get(routes::healthz::health))
        .with_state(Arc::clone(&ctx))
        .nest("/api", routes::users::router(Arc::clone(&ctx)))
        .nest("/api", routes::articles::router(Arc::clone(&ctx)))
        .layer(rate_limit_layer)
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10))
        .layer(cors::layer(config.allowed_origins))
        .layer(SetSensitiveHeadersLayer::new([header::AUTHORIZATION]))
        .layer(CatchPanicLayer::new())
        .split_for_parts();

    // ------------------------ ATTACH DOCUMENTATION ---------------------------
    let oai = OPENAPI_JSON.get_or_init(|| docs.to_json().unwrap().leak());
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
    if config.migrate {
        info!("Applying database migrations");
        sqlx::migrate!()
            .run(&ctx.db)
            .await
            .context("failed to run migrations")?;
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
