use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::State;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::Db;
use crate::http::jwt::{issue_token, verify_token};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct HealthCheckPayload {
    version: &'static str,
}

#[get("/healthz")]
#[instrument(name = "SERVICE HEALTH CHECK", skip(db, encoding_key, decoding_key))]
pub(crate) async fn health(
    db: Connection<Db>,
    encoding_key: &State<EncodingKey>,
    decoding_key: &State<DecodingKey>,
) -> Json<HealthCheckPayload> {
    // token keys are in state and issue/verify works as expected
    let token = issue_token("health-check", encoding_key).expect("issued jwt");
    verify_token(&token, decoding_key).expect("valid jwt");
    // database is accepting connections
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
