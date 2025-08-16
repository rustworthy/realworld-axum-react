use crate::AppContext;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod auth;
mod current;
mod register;

// ---------------------------- SHARED TYPES -----------------------------------
#[derive(Debug, Serialize, ToSchema)]
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
    #[schema(required = true)]
    image: Option<Url>,
}

/// Container for all user management related endpoints.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/endpoints/>
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct UserPayload<U> {
    user: U,
}

// ------------------------------- ROUTER --------------------------------------
pub(crate) fn router(ctx: Arc<AppContext>) -> OpenApiRouter {
    let user_router = OpenApiRouter::new().routes(routes!(
        current::read_current_user,
        current::update_current_user,
    ));

    let auth_router = OpenApiRouter::new()
        .routes(routes!(register::register_user))
        // `routes!` create a method router internally, and since we already
        // got `POST` user registration, this route should be attached via
        // a separate `routes!` call: https://stackoverflow.com/a/79303329
        .routes(routes!(auth::login))
        .routes(routes!(register::confirm_email));

    OpenApiRouter::new()
        .nest("/user", user_router)
        .nest("/users", auth_router)
        .with_state(ctx)
}
