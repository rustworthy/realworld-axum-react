use std::ops::Deref;
use std::sync::Arc;

use crate::AppContext;
use crate::http::errors::Error;
use crate::http::jwt::verify_token;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use uuid::Uuid;

#[derive(Debug)]
pub(in crate::http) struct UserID(pub Uuid);
#[derive(Debug)]
pub(in crate::http) struct MaybeUserID(pub Option<UserID>);

impl Deref for UserID {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRequestParts<S> for UserID
where
    // https://docs.rs/axum/0.6.4/axum/extract/struct.State.html#for-library-authors
    Arc<AppContext>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(token) = utils::maybe_token(&parts.headers) else {
            return Err(Error::Unauthorized);
        };
        let ctx = Arc::<AppContext>::from_ref(state);
        verify_token(token, &ctx.dec_key)
            .map_err(|e| {
                warn!("Authentication failed: {}", e);
                Error::Unauthorized
            })
            .map(UserID)
    }
}

impl<S> FromRequestParts<S> for MaybeUserID
where
    Arc<AppContext>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(token) = utils::maybe_token(&parts.headers) else {
            return Ok(Self(None));
        };
        let ctx = Arc::<AppContext>::from_ref(state);
        let uid = verify_token(token, &ctx.dec_key)
            .map_err(|e| {
                warn!("Authentication failed: {}", e);
                Error::Unauthorized
            })
            .map(UserID)?;
        Ok(Self(Some(uid)))
    }
}

mod utils {
    use axum::http::HeaderMap;

    pub fn maybe_token(headers: &HeaderMap) -> Option<&str> {
        headers
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
    }
}
