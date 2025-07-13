use super::{User, UserPayload};
use crate::db::Db;
use crate::http::guards::UserID;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

/// Read currently logged in user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    tags = ["Users"],
    responses(
        (status = 200, description = "User details and fresh JWT.", body = UserPayload<User>),
        (status = 401, description = "Token missing or invalid."),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "GET CURRENT USER", skip(_db))]
#[get("/user")]
pub(crate) async fn read_current_user(id: UserID, _db: Connection<Db>) -> Json<UserPayload<User>> {
    dbg!(id.0);
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

/// Update currently logged in user.
///
/// This will return user's details and a re-freshed JWT token.
#[utoipa::path(
    tags = ["Users"],
    responses(
        (status = 401, description = "Authentication required."),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "GET CURRENT USER", skip(_db))]
#[put("/user")]
pub(crate) async fn update_current_user(id: UserID, _db: Connection<Db>) {
    dbg!(id.0);
}
