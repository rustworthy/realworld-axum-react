use crate::http::jwt::issue_token;
use crate::{Db, http::guards::UserID};
use jsonwebtoken::EncodingKey;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings};
use uuid::Uuid;

// --------------------------------TYPES ---------------------------------------
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserPayload<U> {
    user: U,
}

#[derive(Debug, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    /// User's email, e.g. `rob.pike@gmail.com`.
    email: String,

    /// Fresh JWT token.
    token: String,

    /// User's name or nickname.
    ///
    /// This is  - just like [`User::email`] - unique in the system.
    username: String,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: String,

    /// Location of user's image (if any).
    image: Option<String>,
}

#[derive(Debug, JsonSchema, Deserialize)]
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

// ------------------------------ HANDLERS -------------------------------------
/// Register new user in the system.
///
/// This will register new user in the system and also create
/// a JWT token, i.e. will immediate log them in.
#[instrument(name = "REGISTER USER", skip_all)]
#[openapi(tag = "User", ignore = "_db")]
#[post("/user", data = "<registration>")]
pub(crate) async fn create_user(
    registration: Json<UserPayload<Registration>>,
    encoding_key: &State<EncodingKey>,
    _db: Connection<Db>,
) -> Json<UserPayload<User>> {
    let user = registration.into_inner().user;
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();
    let jwt_string = issue_token(uid, encoding_key).unwrap();
    {
        user.password // should verify strength, hash, and store
    };
    Json(UserPayload {
        user: User {
            email: user.email,
            token: jwt_string,
            username: user.username,
            bio: "".into(),
            image: None,
        },
    })
}

#[instrument(name = "GET CURRENT USER", skip(_db))]
#[openapi(tag = "User", ignore = "_db")]
#[get("/user")]
pub(crate) async fn read_current_user(
    user_id: UserID,
    _db: Connection<Db>,
) -> Json<UserPayload<User>> {
    dbg!(user_id.0);
    Json(UserPayload {
        user: User {
            email: "pavel@mikhalkevich.com".into(),
            token: "pavel.mikhalkevich.com".into(),
            username: "pavel.mikhalkevich".into(),
            bio: "".into(),
            image: None,
        },
    })
}

#[instrument(name = "UPDATE CURRENT USER", skip(_db))]
#[openapi(tag = "User", ignore = "_db")]
#[put("/user")]
pub(crate) async fn update_current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}

#[instrument(name = "LOG USER IN", skip(_db))]
#[openapi(tag = "User", ignore = "_db")]
#[post("/user/login")]
pub(crate) async fn login(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}

pub(crate) fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_user, read_current_user, update_current_user, login]
}
