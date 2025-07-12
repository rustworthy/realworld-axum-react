use super::{User, UserPayload};
use crate::Db;
use crate::http::errors::Error;
use crate::http::errors::Validation;
use crate::http::jwt::issue_token;
use jsonwebtoken::EncodingKey;
use rocket::State;
use rocket::serde::Deserialize;
use rocket::serde::json::Error as ParseError;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Registration {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    email: String,

    /// User's name or nickname.
    ///
    /// This is  - just like [`User::email`] - unique in the system.
    username: String,

    /// User's password.
    ///
    /// There are currently no limitations on password strength.
    password: String,
}

/// Register new user.
///
/// This will register new user in the system and also create
/// a JWT token, i.e. will immediate log them in.
#[utoipa::path(
        tags = ["Users", "Auth"],
        responses(
            (status = 201, description = "User successfully created", body = UserPayload<User>),
            (status = 422, description = "Missing or invalid registration details", body = Validation),
            (status = 500, description = "Internal server error.",
        ))
    )]
#[instrument(name = "REGISTER USER", skip_all)]
#[post("/user", data = "<registration>")]
pub(crate) async fn handler(
    registration: Result<Json<UserPayload<Registration>>, ParseError<'_>>,
    encoding_key: &State<EncodingKey>,
    _db: Connection<Db>,
) -> Result<Json<UserPayload<User>>, Error> {
    let user = registration?.into_inner().user;
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();
    let jwt_string = issue_token(uid, encoding_key).unwrap();

    drop(user.password); // should verify strength, hash, and store

    Ok(Json(UserPayload {
        user: User {
            email: user.email,
            token: jwt_string,
            username: user.username,
            bio: "".into(),
            image: None,
        },
    }))
}
