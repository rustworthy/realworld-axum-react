use crate::AppContext;
use crate::http::errors::Error;

pub async fn moderate_content(ctx: &AppContext, content: &str, field: &str) -> Result<(), Error> {
    if ctx.skip_content_moderation {
        warn!("content moderation disabled via app configuration");
        return Ok(());
    }

    let verdict = ctx.moderator.moderate(content).await?;
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
