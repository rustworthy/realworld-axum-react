#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

pub mod telemetry;

use chrono::{DateTime, Utc};
use rocket::{Build, Rocket};
use rocket_db_pools::{Connection, Database, sqlx::PgPool};

#[derive(Database)]
#[database("main")]
struct Db(PgPool);

#[get("/healthz")]
#[instrument(name = "SERVICE HEALTH CHECK", skip(db))]
async fn health(db: Connection<Db>) -> String {
    let db_server_time = check_db_conn(db).await;
    info!("Database server time {:?} millis", &db_server_time);
    db_server_time.to_string()
}

#[instrument(name = "CHECK DATABASE CONNECTION", skip(db))]
async fn check_db_conn(mut db: Connection<Db>) -> DateTime<Utc> {
    let time: DateTime<Utc> = sqlx::query!("SELECT NOW()::timestamptz AS db_time;")
        .fetch_one(&mut **db)
        .await
        .expect("successfully fetch timestamp from db engine")
        .db_time
        .unwrap();
    time
}

pub fn construct_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![health])
        .attach(Db::init())
}
