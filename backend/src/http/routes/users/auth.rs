use crate::db::Db;
use crate::http::errors::Validation;
use rocket::http::Status;
use rocket_db_pools::Connection;

/// Log user in.
///
/// This will return user's details as well as a fresh JWT token.
#[utoipa::path(
        tags = ["Users", "Auth"],
        responses(
            (status = 422, description = "Missing or invalid registration details", body = Validation),
            (status = 500, description = "Internal server error."),
        )
)]
#[instrument(name = "LOG USER IN", skip(_db))]
#[post("/user/login")]
pub(crate) async fn login(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}
