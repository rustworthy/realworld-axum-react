#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tracing;

pub mod telemetry;

use chrono::{DateTime, Utc};
use rocket::{Build, Rocket, fairing};
use rocket_db_pools::{Connection, Database, sqlx::PgPool};
use uuid::Uuid;

#[derive(Database)]
#[database("main")]
struct Db(PgPool);

#[get("/healthz")]
#[instrument(name = "SERVICE HEALTH CHECK", skip(db))]
async fn health(db: Connection<Db>) -> String {
    let db_check_payload = check_db_conn(db).await;
    info!(
        "Database server time {:?} millis",
        &db_check_payload.db_time
    );
    format!(
        "DB server time: {}. Nonce={}. MIGRATE={}",
        db_check_payload.db_time,
        db_check_payload.nonce,
        std::env::var("MIGRATE").ok().unwrap_or("0".into())
    )
}

#[derive(Debug)]
struct DatabaseCheckPayload {
    db_time: DateTime<Utc>,
    nonce: Uuid,
}

#[instrument(name = "CHECK DATABASE CONNECTION", skip(db))]
async fn check_db_conn(mut db: Connection<Db>) -> DatabaseCheckPayload {
    let payload = sqlx::query_as!(
        DatabaseCheckPayload,
        r#"
        SELECT
            NOW()::timestamptz AS "db_time!",
            uuid_generate_v4() AS "nonce!"
        ;
    "#
    )
    .fetch_one(&mut **db)
    .await
    .expect("successfully fetch data db engine");
    payload
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
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

pub fn construct_rocket(migrate: bool) -> Rocket<Build> {
    let rocket = rocket::build()
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
