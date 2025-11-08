use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use axum::response::AppendHeaders;
use axum::response::IntoResponse as _;
use axum::response::Response;
use deadpool_redis::Pool as RedisPool;
use tower_redis_cell::Error as RateLimitError;
use tower_redis_cell::ProvideRuleError;
use tower_redis_cell::RateLimitConfig;
use tower_redis_cell::deadpool::RateLimitLayer;
use tower_redis_cell::redis_cell::{Key, Policy};
use tower_redis_cell::{ProvideRule, ProvideRuleResult, Rule};

pub(crate) const BASIC_POLICY: Policy = Policy::from_tokens_per_minute(120)
    .max_burst(120)
    .name("basic");
pub(crate) const STRICT_POLICY: Policy = Policy::from_tokens_per_day(5).max_burst(5).name("strict");
pub(crate) const VERY_STRICT_POLICY: Policy = Policy::from_tokens_per_day(1)
    .max_burst(1)
    .name("very_strict");

#[derive(Clone, Debug, Default)]
pub(crate) struct RuleProvider {
    skip_rate_limiting: bool,
}

impl<T> ProvideRule<Request<T>> for RuleProvider {
    fn provide<'a>(&self, req: &'a Request<T>) -> ProvideRuleResult<'a> {
        // we want to accomodate for the end-to-end test suites
        if self.skip_rate_limiting {
            return Ok(None);
        }

        let (path, method) = (req.uri().path(), req.method());

        // the app is sitting behind `Kamal-Proxy` which sets 'x-forwarded-for' for us:
        // https://kamal-deploy.org/docs/configuration/proxy/#forward-headers
        let ip = match req.headers().get("x-forwarded-for") {
            None => {
                // we are developing locally w/o reverse-proxy
                if cfg!(debug_assertions) {
                    "localhost"
                // the `Kamal-Proxy` itself is monitoring the app's health,
                // and - since this is the only route that `Kamal-Proxy` might
                // be pinging - we can return the rule right away
                } else if path == "healthz" {
                    return Ok(Some(Rule::new("proxy-healthcheck", BASIC_POLICY)));
                } else {
                    return Err("'x-forwarded-for' header is missing".into());
                }
            }
            Some(ip_header) => ip_header
                .to_str()
                .map_err(|e| ProvideRuleError::default().detail(e.to_string()))?,
        };

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
        Ok(Some(Rule::new(key, BASIC_POLICY)))
    }
}

pub(crate) fn rate_limit_layer<T>(
    pool: RedisPool,
    skip_rate_limiting: bool,
) -> anyhow::Result<RateLimitLayer<RuleProvider, Request<T>, Response, Response>> {
    let provider = RuleProvider { skip_rate_limiting };
    let rate_limit_config: RateLimitConfig<RuleProvider, Request<T>, Response, Response> =
        RateLimitConfig::new(provider, |err, _req: &Request<T>| match err {
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
        });

    Ok(RateLimitLayer::new(rate_limit_config, pool))
}
