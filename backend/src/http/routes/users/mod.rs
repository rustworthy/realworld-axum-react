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
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    username: String,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: String,

    /// Location of user's image (if any).
    image: Option<String>,
}

/// Container for all user management related endpoints.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/endpoints/>
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserPayload<U> {
    user: U,
}

// ------------------------------ HANDLERS -------------------------------------

pub(crate) fn routes() -> Vec<Route> {
    routes![
        register::register_user,
        current::read_current_user,
        current::update_current_user,
        auth::login
    ]
}

// -------------------------------- DOCS ---------------------------------------
#[derive(OpenApi)]
#[openapi(
    paths(
        register::register_user,
        current::read_current_user,
        current::update_current_user,
        auth::login,
    ),
    tags(
        (name = "Users", description = "User management endpoints."),
    ),
)]
pub(crate) struct UserApiDocs;
