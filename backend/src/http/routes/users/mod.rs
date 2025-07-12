use rocket::Route;
use rocket::serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

mod auth;
mod current;
mod register;

// ---------------------------- SHARED TYPES -----------------------------------
#[derive(Debug, Serialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserPayload<U> {
    user: U,
}

// ------------------------------ HANDLERS -------------------------------------

pub(crate) fn routes() -> Vec<Route> {
    routes![
        register::handler,
        current::read,
        current::update,
        auth::login
    ]
}

// -------------------------------- DOCS ---------------------------------------
#[derive(OpenApi)]
#[openapi(
    paths(
        register::handler,
        current::read,
        current::update,
        auth::login,
    ),
    tags(
        (name = "Users", description = "User management endpoints."),
    ),
)]
pub(crate) struct UserApiDocs;
