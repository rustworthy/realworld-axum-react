use axum::http::{Method, header};
use regex::RegexSet;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub(crate) fn layer<I, S>(allowed_origins: I) -> CorsLayer
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let origins = RegexSet::new(allowed_origins).expect("valid expressions");
    CorsLayer::new()
        .allow_methods([Method::GET, Method::PATCH, Method::PUT])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin(AllowOrigin::predicate(move |origin, _| {
            origin.to_str().is_ok_and(|o| origins.is_match(o))
        }))
}
