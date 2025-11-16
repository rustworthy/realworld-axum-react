use anyhow::Context;
use apalis::layers::WorkerBuilderExt;
use apalis::prelude::*;
use sqlx::PgPool;
use std::str::FromStr;

const MAINTENANCE_SCHEDULE: &str =
    /*
    see: https://docs.rs/cron/latest/cron/

    sec  min  hour  day of month  month   day of week   year
    */
    "0   0    0     *             *       *             *";

pub(crate) async fn monitor(db: PgPool, redis_url: &str) -> anyhow::Result<Monitor> {
    // Apalis will instantiate `redis::aio::ConnectionManager` here, which is
    // similar to the connections that our Redis pool holds (since they are
    // all `aio::ConnectionLike`), but it also auto-reconnects when needed
    let conn = apalis_redis::connect(redis_url)
        .await
        .context("Apalis to have created redis::aio::ConnectionManager")?;
    let redis_storage = apalis_redis::RedisStorage::new(conn);

    let schedule =
        apalis_cron::Schedule::from_str(MAINTENANCE_SCHEDULE).context("invalid cron expression")?;
    let cron_stream = apalis_cron::CronStream::new(schedule);
    let backend = cron_stream.pipe_to_storage(redis_storage);

    let worker = WorkerBuilder::new("confirmation_tokens_maintenance")
        .enable_tracing()
        .data(db)
        .backend(backend)
        .build_fn(|_job: (), data: Data<PgPool>| async move {
            utils::clean_up_confirmation_tokens(&data).await
        });

    Ok(Monitor::new().register(worker))
}

mod utils {
    use anyhow::Context as _;
    use sqlx::PgPool;

    pub(super) async fn clean_up_confirmation_tokens(pool: &PgPool) -> anyhow::Result<u64> {
        let ndeleted = sqlx::query!("DELETE FROM confirmation_tokens WHERE expires_at >= NOW() ")
            .execute(pool)
            .await
            .context("error occurred when cleaning up confirmation_tokens table")?
            .rows_affected();
        info!(ndeleted, "confirmation_tokens table maintenance");
        Ok(ndeleted)
    }
}
