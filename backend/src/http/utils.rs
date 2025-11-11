use crate::AppContext;
use crate::http::errors::Error;
use crate::services::moderator::Verdict;
use crate::utils;
use anyhow::Context as _;
use deadpool_redis::redis::Cmd as RedisCmd;
use deadpool_redis::redis::ErrorKind as RedisErrorKind;
use deadpool_redis::redis::FromRedisValue;
use deadpool_redis::redis::RedisResult;
use deadpool_redis::redis::Value as RedisValue;

pub async fn moderate_content(ctx: &AppContext, content: &str, field: &str) -> Result<(), Error> {
    if ctx.skip_content_moderation {
        warn!("content moderation disabled via app configuration");
        return Ok(());
    }

    let verdict = {
        // using a colon as per the naming convention:
        // https://redis.io/docs/latest/develop/using-commands/keyspace/#content-of-keys
        let content_key = format!("moderation:{}", utils::md5_hash(content));
        match restore_verdict(&content_key, ctx).await? {
            Some(cached_verdict) => cached_verdict,
            None => {
                let verdict = ctx.moderator.moderate(content).await?;
                cache_verdict(&content_key, &verdict, ctx).await?;
                verdict
            }
        }
    };

    if !verdict.processable {
        return Err(Error::unprocessable_entity([(
            field,
            r#"Please make sure image content is formatted correctly
            and links are valid and accessible."#,
        )]));
    }

    if verdict.flagged {
        warn!(
            content = content,
            flagged = true,
            details = serde_json::to_string(&verdict.details).ok(),
            "content flagged"
        );
        return Err(Error::unprocessable_entity([(
            field,
            r#"Please make sure there is no violent or otherwise
            indecent content in text and/or images"#,
        )]));
    }

    Ok(())
}

pub async fn cache_verdict(
    content_key: &str,
    verdict: &Verdict,
    ctx: &AppContext,
) -> anyhow::Result<()> {
    let mut conn = ctx
        .redis
        .get()
        .await
        .context("failed to acquire Redis connenction from pool")?;
    let verdict_string = serde_json::to_string(&verdict).context("failed to serialize verdict")?;
    RedisCmd::set_ex(
        content_key,
        verdict_string,
        60 * 60 * 24 * 30, // for a month
    )
    .exec_async(&mut conn)
    .await
    .context("failed to cache verdict")?;

    Ok(())
}

pub async fn restore_verdict(
    content_key: &str,
    ctx: &AppContext,
) -> anyhow::Result<Option<Verdict>> {
    let mut conn = ctx
        .redis
        .get()
        .await
        .context("failed to acquire Redis connenction from pool")?;
    let value: RedisValue = RedisCmd::get(content_key)
        .query_async(&mut conn)
        .await
        .context("failed to get value from Redis")?;
    let result: MaybeVerdict =
        FromRedisValue::from_redis_value(&value).context("failed to deserialize verdict")?;
    Ok(result.0)
}

struct MaybeVerdict(Option<Verdict>);

impl FromRedisValue for MaybeVerdict {
    fn from_redis_value(v: &RedisValue) -> RedisResult<Self> {
        let verdict = match v {
            RedisValue::Nil => None,
            RedisValue::BulkString(value) => {
                let verdict: Verdict = serde_json::from_slice(value).map_err(|e| {
                    (
                        RedisErrorKind::ParseError,
                        "verdict deserialization falied",
                        format!("failed to deserialize as Verdict: {}", e),
                    )
                })?;
                Some(verdict)
            }
            _ => {
                return Err((
                    RedisErrorKind::ParseError,
                    "verdict deserialization falied",
                    format!("Expected an optional bulk string, got: {:?}", v),
                )
                    .into());
            }
        };
        Ok(MaybeVerdict(verdict))
    }
}
