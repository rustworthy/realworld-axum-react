use crate::{Db, http::guards::UserID};
use rocket::http::Status;
use rocket_db_pools::Connection;

#[post("/")]
#[instrument(name = "REGISTER USER", skip(_db))]
pub(crate) async fn create_user(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}

#[get("/")]
#[instrument(name = "GET CURRENT USER", skip(_db))]
pub(crate) async fn read_current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}

#[put("/")]
#[instrument(name = "UPDATE CURRENT USER", skip(_db))]
pub(crate) async fn update_current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}

#[post("/login")]
#[instrument(name = "LOG USER IN", skip(_db))]
pub(crate) async fn login(_db: Connection<Db>) -> Status {
    Status::UnprocessableEntity
}
