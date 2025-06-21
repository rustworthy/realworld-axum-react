#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

mod routes;
mod telemetry;

use rocket_db_pools::{Database, sqlx::PgPool};
pub use telemetry::init_tracing;

use crate::routes::health;
use rocket::{Build, Rocket, fairing};

#[derive(Database)]
#[database("main")]
pub(crate) struct Db(PgPool);

pub(crate) async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let Some(db) = Db::fetch(&rocket) else {
        return Err(rocket);
    };
    match sqlx::migrate!().run(&**db).await {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to migrate database: {}", e);
            Err(rocket)
        }
    }
}

pub fn construct_rocket(database_url: String, migrate: bool) -> Rocket<Build> {
    let config = rocket::Config::figment().merge(("databases.main.url", database_url));
    let rocket = rocket::custom(config)
        .mount("/", routes![health])
        .attach(Db::init());

    match migrate {
        true => rocket.attach(fairing::AdHoc::try_on_ignite(
            "Run pending database migrations",
            run_migrations,
        )),
        false => rocket,
    }
}
