use std::sync::Arc;

use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::jwt::issue_token;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct Login {
    /// User's email, e.g. `rob.pike@gmail.com`.
    ///
    /// This is case-insensitively unique in the system.
    email: String,

    /// User's password.
    password: String,
}

/// Log user in.
///
/// This will return user's details as well as a fresh JWT token.
#[utoipa::path(
    post,
    path = "/login",
    tags = ["Users"],
    responses(
        (status = 200, description = "User successfully logged in", body = UserPayload<User>),
        (status = 422, description = "Missing or invalid login details", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(/* authentication NOT required */),
)]
#[instrument(name = "LOG USER IN", skip_all)]
pub(crate) async fn login(
    ctx: State<Arc<AppContext>>,
    login_details: Result<Json<UserPayload<Login>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = login_details?;
    // @Dzmitry, we of course should not be just dropping user's password,
    // rather should verify it's not empty, hash it and compare to what is
    // stored in our database
    drop(user.password);

    // @Dzmitry as if we found a user in db
    let uid = Uuid::parse_str("25f75337-a5e3-44b1-97d7-6653ca23e9ee").unwrap();

    // @Dzmitry and we issued a token for the newly created user
    let jwt_string = issue_token(uid, &ctx.enc_key).unwrap();

    let paylaod = UserPayload {
        user: User {
            email: user.email,
            token: jwt_string,
            username: "rob.pike".into(),
            bio: "Co-author Go programming language".into(),
            image: Some("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT_ySzB8CjQ85dLtvWXX8K1F4RlxbPEzjgfgKNTwneiPUCyfixt4edM8Nc&s".parse().expect("a valid URL")),
        },
    };

    Ok(Json(paylaod))
}
