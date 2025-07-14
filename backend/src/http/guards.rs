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
pub(in crate::http) struct UserID(pub Uuid);

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
    type Error = ();

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
}

// --------------------------- SECURITY ADDON ----------------------------------
use utoipa::Modify;
use utoipa::openapi::security::Http;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::SecurityScheme;

pub(crate) struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("some components to be have been registered");
        components.add_security_scheme(
            "HttpAuthBearerJWT",
            SecurityScheme::Http(
                Http::builder()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JSON web token string in Authorization header"))
                    .build(),
            ),
        )
    }
}
