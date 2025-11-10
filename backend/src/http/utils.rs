use crate::AppContext;
use crate::http::errors::Error;
use crate::services::moderator::Verdict;
use crate::utils;
use anyhow::Context as _;
use deadpool_redis::redis::Cmd as RedisCmd;

pub async fn moderate_content(ctx: &AppContext, content: &str, field: &str) -> Result<(), Error> {
    if ctx.skip_content_moderation {
        warn!("content moderation disabled via app configuration");
        return Ok(());
    }

    let verdict = ctx.moderator.moderate(content).await?;
    cache_verdict(content, &verdict, ctx).await?;

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
    content: &str,
    verdict: &Verdict,
    ctx: &AppContext,
) -> anyhow::Result<()> {
    let mut conn = ctx
        .redis
        .get()
        .await
        .context("failed to acquire Redis connenction from pool")?;
    let content_key = format!("moderation::{}", utils::md5_hash(content));
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
