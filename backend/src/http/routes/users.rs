use crate::{Db, http::guards::UserID};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings};

#[openapi(tag = "User", ignore = "_db")]
#[post("/")]
#[instrument(name = "REGISTER USER", skip(_db))]
pub(crate) async fn create_user(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}

#[openapi(tag = "User", ignore = "_db", ignore = "user_id")]
#[get("/")]
#[instrument(name = "GET CURRENT USER", skip(_db))]
pub(crate) async fn read_current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}

#[openapi(tag = "User", ignore = "_db", ignore = "user_id")]
#[put("/")]
#[instrument(name = "UPDATE CURRENT USER", skip(_db))]
pub(crate) async fn update_current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}

#[openapi(tag = "User", ignore = "_db")]
#[post("/login")]
#[instrument(name = "LOG USER IN", skip(_db))]
pub(crate) async fn login(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}

pub(crate) fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_user, read_current_user, update_current_user, login]
}
