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

// ------------------------- OPENAPI DESCRIPTION  ------------------------------
// See example implementation in `okapi` crate:
// https://github.com/GREsau/okapi/blob/e5146ea4303743d63704f26db600c6b3e9cd8294/examples/secure_request_guard/src/http_auth.rs

use rocket_okapi::r#gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::request::RequestHeaderInput;

impl OpenApiFromRequest<'_> for UserID {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires a JWT token to access".into()),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(), // `basic`, `digest`, ...
                bearer_format: Some("Bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
