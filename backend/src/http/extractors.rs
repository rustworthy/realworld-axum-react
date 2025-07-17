use crate::AppContext;
use crate::http::errors::Error;
use crate::http::jwt::verify_token;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use jsonwebtoken::DecodingKey;
use uuid::Uuid;

#[derive(Debug)]
pub(in crate::http) struct UserID(pub Uuid);

impl<S> FromRequestParts<S> for UserID
where
    S: Send + Sync,
{
    type Rejection = Error;

    #[allow(unused_variables)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
    /*
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let key = try_outcome!(request.guard::<&State<DecodingKey>>().await);
        let Some(token) = request
            .headers()
            .get_one("Authorization")
            .and_then(|header| {
                if header.starts_with("Token") {
                    header.strip_prefix("Token ")
                } else if header.starts_with("Bearer") {
                    header.strip_prefix("Bearer ")
                } else {
                    None
                }
            })
        else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        match verify_token(token, key) {
            Ok(sub) => Outcome::Success(UserID(sub)),
            Err(e) => {
                warn!("Authentication failed: {}", e);
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
    */
}
