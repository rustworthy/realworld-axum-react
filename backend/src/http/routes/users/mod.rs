use crate::AppContext;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod auth;
mod current;
mod profiles;
mod register;
pub(crate) mod utils;

// ---------------------------- SHARED TYPES -----------------------------------
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct User {
    /// User's email, e.g. `rob.pike@gmail.com`.
    #[schema(example = "rob.pike@gmail.com", format = "email")]
    email: String,

    /// Fresh JWT token.
    #[schema(format = "jwt")]
    token: String,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(example = "rob.pike1984")]
    username: String,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: String,

    /// Location of user's image (if any).
    #[schema(required = true)]
    image: Option<Url>,
}

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct UserProfile {
    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(example = "rob.pike1984")]
    username: String,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: String,

    /// Location of user's image (if any).
    #[schema(required = true)]
    image: Option<Url>,

    /// Following, if the current user is subscribed to the searched user
    following: bool,
}

/// Container for all user management related endpoints.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/endpoints/>
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct UserPayload<U> {
    user: U,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct UserProfilePayload<U> {
    profile: U,
}

// ------------------------------- ROUTER --------------------------------------
pub(crate) fn router(ctx: Arc<AppContext>) -> OpenApiRouter {
    let user_router = OpenApiRouter::new().routes(routes!(
        current::read_current_user,
        current::update_current_user,
    ));

    let user_profile = OpenApiRouter::new().routes(routes!(
        profiles::profile,
        profiles::follow_profile,
        profiles::unfollow_profile,
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
        .nest("/profiles", user_profile)
        .with_state(ctx)
}
