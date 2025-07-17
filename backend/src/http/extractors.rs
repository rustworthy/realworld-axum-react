use crate::AppContext;
use crate::http::errors::Error;
use crate::http::jwt::verify_token;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use uuid::Uuid;

#[derive(Debug)]
pub(in crate::http) struct UserID(pub Uuid);

impl<S> FromRequestParts<S> for UserID
where
    // https://docs.rs/axum/0.6.4/axum/extract/struct.State.html#for-library-authors
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(token) = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|value| {
                if value.starts_with("Token") {
                    value.strip_prefix("Token ")
                } else if value.starts_with("Bearer") {
                    value.strip_prefix("Bearer ")
                } else {
                    None
                }
            })
        else {
            return Err(Error::Unauthorized);
        };
        let ctx = AppContext::from_ref(state);
        verify_token(token, &ctx.dec_key)
            .map_err(|e| {
                warn!("Authentication failed: {}", e);
                Error::Unauthorized
            })
            .map(|sub| UserID(sub))
    }
}
