use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::Db;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct HealthCheckPayload {
    version: &'static str,
}

#[get("/healthz")]
#[instrument(name = "SERVICE HEALTH CHECK", skip(db))]
pub(crate) async fn health(db: Connection<Db>) -> Json<HealthCheckPayload> {
    let db_check_payload = check_db_conn(db).await;
    info!("Database server time {:?}", &db_check_payload.db_time);
    Json(HealthCheckPayload {
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Debug)]
struct DatabaseCheckPayload {
    db_time: DateTime<Utc>,
    // this is solely used to make sure the initial
    // migration with "uuid-ossp" extension was successful
    _nonce: Uuid,
}

#[instrument(name = "CHECK DATABASE CONNECTION", skip(db))]
async fn check_db_conn(mut db: Connection<Db>) -> DatabaseCheckPayload {
    sqlx::query_as!(
        DatabaseCheckPayload,
        r#"SELECT NOW() AS "db_time!", uuid_generate_v4() AS "_nonce!";"#
    )
    .fetch_one(&mut **db)
    .await
    .expect("successfully fetch data db engine")
}
