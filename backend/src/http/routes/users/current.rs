use super::{User, UserPayload};
use crate::AppContext;
use crate::http::errors::{Error, Validation};
use crate::http::extractors::UserID;
use crate::http::jwt::issue_token;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use std::sync::Arc;
use url::Url;
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
#[axum::debug_handler]
pub(crate) async fn read_current_user(
    ctx: State<Arc<AppContext>>,
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
    #[schema(nullable = false, example = "rob.pike@gmail.com", format = "email")]
    email: Option<String>,

    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(nullable = false, example = "rob.pike1984")]
    username: Option<String>,

    /// User's biography.
    ///
    /// Note that Empty string will override the existing biography.
    #[schema(
        nullable = false,
        example = "In 2007, while working at Google, I designed Go together with Robert Griesemer and Ken Thompson"
    )]
    bio: Option<String>,

    /// New password.
    #[schema(nullable = false, min_length = 1, example = "Whoami@g00gle")]
    password: Option<String>,

    /// New image URL.
    ///
    /// Specifying `null` means removing the image altogether.
    #[serde(default, with = "::serde_with::rust::double_option")]
    image: Option<Option<Url>>,
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
    ctx: State<Arc<AppContext>>,
    id: UserID,
    input: Result<Json<UserPayload<UserUpdate>>, JsonRejection>,
) -> Result<Json<UserPayload<User>>, Error> {
    let Json(UserPayload { user }) = input?;
    drop(user.password);
    let (email, username, image, bio) = if let Some(image_url) = user.image {
        // TODO: let's include image URL update into the user update query
        (
            user.email.unwrap_or("r.pike@gmail.com".to_string()),
            user.username.unwrap_or("r.pike1994".into()),
            image_url,
            user.bio.unwrap_or_default(),
        )
    } else {
        // TODO: without image update;
        (
            user.email.unwrap_or("r.pike@gmail.com".to_string()),
            user.username.unwrap_or("r.pike10984".into()),
            None,
            user.bio.unwrap_or_default(),
        )
    };

    let jwt_string = issue_token(id.0, &ctx.enc_key).unwrap();
    let payload = UserPayload {
        user: User {
            email,
            token: jwt_string,
            username,
            bio,
            image,
        },
    };
    Ok(Json(payload))
}
