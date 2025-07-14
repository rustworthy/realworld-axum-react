use super::{User, UserPayload};
use crate::db::Db;
use crate::http::errors::Error;
use crate::http::errors::Validation;
use crate::http::guards::UserID;
use crate::http::jwt::issue_token;
use jsonwebtoken::EncodingKey;
use rocket::State;
use rocket::serde::Deserialize;
use rocket::serde::json::Error as JsonError;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use utoipa::ToSchema;

/// Read currently logged in user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Token missing or invalid."),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "GET CURRENT USER", skip(_db, encoding_key))]
#[get("/user")]
pub(crate) async fn read_current_user(
    id: UserID,
    encoding_key: &State<EncodingKey>,
    _db: Connection<Db>,
) -> Json<UserPayload<User>> {
    let jwt_string = issue_token(id.0, encoding_key).unwrap();
    Json(UserPayload {
        user: User {
            email: "pavel@mikhalkevich.com".into(),
            token: jwt_string,
            username: "pavel.mikhalkevich".into(),
            bio: "".into(),
            image: None,
        },
    })
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserUpdate {
    /// User's email, e.g. `rob.pike@gmail.com`.
    email: Option<String>,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    username: Option<String>,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: Option<String>,

    /// Location of user's image (if any).
    image: Option<String>,
}

/// Update currently logged in user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Authentication required."),
        (status = 422, description = "Missing or invalid registration details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "GET CURRENT USER", skip(_db, encoding_key))]
#[put("/user", data = "<update>")]
pub(crate) async fn update_current_user(
    id: UserID,
    update: Result<Json<UserPayload<UserUpdate>>, JsonError<'_>>,
    encoding_key: &State<EncodingKey>,
    _db: Connection<Db>,
) -> Result<Json<UserPayload<User>>, Error> {
    let jwt_string = issue_token(id.0, encoding_key).unwrap();
    let partial = update?.into_inner().user;
    Ok(Json(UserPayload {
        user: User {
            email: partial.email.unwrap_or("pavel@mikhalkevich.com".into()),
            token: jwt_string,
            username: partial.username.unwrap_or("pavel.mikhalkevich".into()),
            bio: partial.bio.unwrap_or("".into()),
            image: partial.image,
        },
    }))
}
