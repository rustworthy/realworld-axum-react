use crate::{http::errors::Error, state::AppContext};

#[instrument(name = "VERIFY CAPTCHA", skip_all)]
pub async fn check_captcha(captcha: Option<String>, ctx: &AppContext) -> Result<(), Error> {
    match captcha {
        Some(token) => {
            let result = ctx.captcha.verify(token).await?;
            if !result.success {
                Err(Error::unprocessable_entity([(
                    "captcha",
                    "invalid or expired",
                )]))
            } else {
                Ok(())
            }
        }
        None if ctx.skip_captcha_verification => Err(Error::unprocessable_entity([(
            "captcha",
            "cannot be empty",
        )])),
        _ => Ok(()),
    }
}
