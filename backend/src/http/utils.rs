use crate::AppContext;
use crate::http::errors::Error;
use crate::services::moderator::Verdict;
use crate::utils;
use deadpool_redis::redis::ErrorKind as RedisErrorKind;
use deadpool_redis::redis::FromRedisValue;
use deadpool_redis::redis::RedisResult;
use deadpool_redis::redis::Value as RedisValue;
use std::time::Duration;

const VERDICT_CACHE_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 30);

pub async fn moderate_content(ctx: &AppContext, content: &str, field: &str) -> Result<(), Error> {
    if ctx.skip_content_moderation {
        warn!("content moderation disabled via app configuration");
        return Ok(());
    }

    let verdict = {
        // using a colon as per the naming convention:
        // https://redis.io/docs/latest/develop/using-commands/keyspace/#content-of-keys
        let content_key = format!("moderation:{}", utils::md5_hash(content));
        match ctx.cache.get::<MaybeVerdict>(&content_key).await?.0 {
            Some(cached_verdict) => cached_verdict,
            None => {
                let verdict = ctx.moderator.moderate(content).await?;
                ctx.cache
                    .set(&content_key, &verdict, Some(VERDICT_CACHE_TTL))
                    .await?;
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

struct MaybeVerdict(Option<Verdict>);

impl FromRedisValue for MaybeVerdict {
    fn from_redis_value(v: &RedisValue) -> RedisResult<Self> {
        let verdict = match v {
            RedisValue::Nil => None,
            RedisValue::BulkString(value) => {
                let verdict: Verdict = serde_json::from_slice(value).map_err(|e| {
                    (
                        RedisErrorKind::ParseError,
                        "verdict deserialization failed",
                        format!("failed to deserialize as Verdict: {}", e),
                    )
                })?;
                Some(verdict)
            }
            _ => {
                return Err((
                    RedisErrorKind::ParseError,
                    "verdict deserialization failed",
                    format!("Expected an optional bulk string, got: {:?}", v),
                )
                    .into());
            }
        };
        Ok(MaybeVerdict(verdict))
    }
}
