use crate::{Db, http::guards::UserID};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings};

// --------------------------------TYPES ---------------------------------------
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserPayload {
    user: User,
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    /// User's email, e.g. `rob.pike@gmail.com`.
    email: String,

    /// Fresh JWT token.
    token: String,

    /// User's name or nickname.name,
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

// ------------------------------ HANDLERS -------------------------------------
#[instrument(name = "REGISTER USER", skip(_db))]
#[openapi(tag = "User", ignore = "_db")]
#[post("/user")]
pub(crate) async fn create_user(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}

#[instrument(name = "GET CURRENT USER", skip(_db))]
#[openapi(tag = "User", ignore = "_db")]
#[get("/user")]
pub(crate) async fn read_current_user(_db: Connection<Db>, user_id: UserID) -> Json<UserPayload> {
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
