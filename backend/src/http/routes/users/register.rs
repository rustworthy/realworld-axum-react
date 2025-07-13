use super::{User, UserPayload};
use crate::db::Db;
use crate::http::errors::Error;
use crate::http::errors::Validation;
use crate::http::jwt::issue_token;
use jsonwebtoken::EncodingKey;
use rocket::State;
use rocket::serde::Deserialize;
use rocket::serde::json::Error as JsonError;
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
        tags = ["Users"],
        responses(
            (status = 201, description = "User successfully created", body = UserPayload<User>),
            (status = 422, description = "Missing or invalid registration details", body = Validation),
            (status = 500, description = "Internal server error.",
        ))
    )]
#[instrument(name = "REGISTER USER", skip_all)]
#[post("/user", data = "<registration>")]
pub(crate) async fn register_user(
    registration: Result<Json<UserPayload<Registration>>, JsonError<'_>>,
    encoding_key: &State<EncodingKey>,
    _db: Connection<Db>,
) -> Result<Json<UserPayload<User>>, Error> {
    let user = registration?.into_inner().user;

    // @Dzmitry, we of course should not be just dropping user's password,
    // rather should verify it's not empty, hash, and store it in the database
    // we already got hashing function in the codebase, but we do not have
    // `user` table, neither sqlx query. It is the database engine that will
    // issue uuid and return it back to us.
    drop(user.password);

    // @Dzmitry as if db engine returned this UUID to us
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();

    // @Dzmitry and we issued a token for the newly created user
    let jwt_string = issue_token(uid, encoding_key).unwrap();

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
