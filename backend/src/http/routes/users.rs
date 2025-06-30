use rocket_db_pools::Connection;

use crate::{Db, http::guards::UserID};

#[get("/")]
#[instrument(name = "GET CURRENT USER", skip(_db))]
pub(crate) async fn current_user(_db: Connection<Db>, user_id: UserID) {
    dbg!(user_id.0);
}
