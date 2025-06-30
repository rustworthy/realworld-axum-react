use crate::http::jwt::verify_token;
use jsonwebtoken::DecodingKey;
use rocket::State;
use rocket::outcome::try_outcome;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserID(pub Uuid);

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let key = try_outcome!(request.guard::<&State<DecodingKey>>().await);
        match verify_token("1312312", key) {
            Ok(sub) => Outcome::Success(UserID(sub)),
            Err(e) => {
                warn!("Authentication failed: {}", e);
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}
