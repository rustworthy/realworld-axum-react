use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::extractors::UserID;
use crate::http::jwt::issue_token;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use utoipa::ToSchema;

/// Read current user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    get,
    path = "",
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Token missing or invalid."),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "GET CURRENT USER", skip(ctx))]
pub(crate) async fn read_current_user(
    ctx: State<AppContext>,
    id: UserID,
) -> Result<Json<UserPayload<User>>, Error> {
    let jwt_string = issue_token(id.0, &ctx.enc_key).unwrap();
    let payload = UserPayload {
        user: User {
            email: "pavel@mikhalkevich.com".into(),
            token: jwt_string,
            username: "pavel.mikhalkevich".into(),
            bio: "".into(),
            image: None,
        },
    };
    Ok(Json(payload))
}

#[derive(Debug, Deserialize, ToSchema)]
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
}

/// Update current user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    put,
    path = "",
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Authentication required."),
        (status = 422, description = "Missing or invalid registration details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "UPDATE CURRENT USER", skip(ctx))]
pub(crate) async fn update_current_user(
    ctx: State<AppContext>,
    id: UserID,
    input: Result<Json<UserPayload<UserUpdate>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;
    let jwt_string = issue_token(id.0, &ctx.enc_key).unwrap();
    let payload = UserPayload {
        user: User {
            email: user.email.unwrap_or("pavel@mikhalkevich.com".into()),
            token: jwt_string,
            username: user.username.unwrap_or("pavel.mikhalkevich".into()),
            bio: user.bio.unwrap_or("".into()),
            image: None,
        },
    };
    Ok(Json(payload))
}
