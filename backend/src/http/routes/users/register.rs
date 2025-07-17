use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::jwt::issue_token;
use crate::http::routes::users::UserEndpointResult;
use axum::Json;
use axum::extract::State;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct Registration {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    email: String,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
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
    post,
    path = "",
    tags = ["Users"],
    responses(
        (status = 201, description = "User successfully created", body = UserPayload<User>),
        (status = 422, description = "Missing or invalid registration details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(/* authentication NOT required */),
)]
#[instrument(name = "REGISTER USER", skip_all)]
pub(crate) async fn register_user(
    ctx: State<AppContext>,
    Json(registration): Json<UserPayload<Registration>>,
) -> UserEndpointResult {
    // @Dzmitry, we of course should not be just dropping user's password,
    // rather should verify it's not empty, hash, and store it in the database
    // we already got hashing function in the codebase, but we do not have
    // `user` table, neither sqlx query. It is the database engine that will
    // issue uuid and return it back to us.
    drop(registration.user.password);

    // @Dzmitry as if db engine returned this UUID to us
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();

    // @Dzmitry and we issued a token for the newly created user
    let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();

    let payload = UserPayload {
        user: User {
            email: registration.user.email,
            token: jwt_string,
            username: registration.user.username,
            bio: "".into(),
            image: None,
        },
    };

    Ok(Json(payload))
}
