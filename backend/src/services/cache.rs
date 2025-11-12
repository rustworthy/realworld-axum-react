use anyhow::Context as _;
use deadpool_redis::Connection;
use deadpool_redis::Pool as RedisPool;
use deadpool_redis::redis::Cmd as RedisCmd;
use deadpool_redis::redis::FromRedisValue;
use deadpool_redis::redis::Value as RedisValue;
use serde::Serialize;
use std::time::Duration;

pub struct Cache {
    pool: RedisPool,
}

impl Cache {
    async fn connection(&self) -> anyhow::Result<Connection> {
        self.pool
            .get()
            .await
            .context("failed to acquire Redis connection from pool")
    }

    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let value = serde_json::to_string(&value)?;
        let cmd = if let Some(ttl) = ttl {
            RedisCmd::set_ex(key, value, ttl.as_secs())
        } else {
            RedisCmd::set(key, value)
        };
        let mut conn = self.connection().await?;
        cmd.exec_async(&mut conn)
            .await
            .context("Redis command failed")?;
        Ok(())
    }

    pub async fn get<T>(&self, key: &str) -> anyhow::Result<T>
    where
        T: FromRedisValue,
    {
        let mut conn = self.connection().await?;
        let value: RedisValue = RedisCmd::get(key)
            .query_async(&mut conn)
            .await
            .context("Redis command failed")?;
        let result: T = FromRedisValue::from_redis_value(&value)?;
        Ok(result)
    }
}
