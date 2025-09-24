use crate::{http::errors::Error, state::AppContext};
use url::Url;

#[instrument(name = "VERIFY CAPTCHA", skip_all)]
pub async fn check_captcha(captcha: Option<String>, ctx: &AppContext) -> Result<(), Error> {
    match captcha {
        Some(token) => {
            let result = ctx.captcha.verify(&token).await?;
            if !result.success {
                warn!(
                    failed_token = token,
                    error_codes = ?result.error_codes,
                    "captcha verification failed"
                );
                Err(Error::unprocessable_entity([(
                    "captcha",
                    "invalid or expired",
                )]))
            } else {
                Ok(())
            }
        }
        None if ctx.skip_captcha_verification => Ok(()),
        None => Err(Error::unprocessable_entity([(
            "captcha",
            "cannot be empty",
        )])),
    }
}

/// Parse stored image location as URL.
///
/// We are stroring the location of their image (if any) as a raw string
/// in the database, but our API promises this to be a valid URL. This is
/// unlikely to ever fail since we are the only ones storing that image path
/// in our database and we also make sure to valide that string before persiting
/// it. However, the convertion _is_ fallible and so we using this helper.
///
/// As a side-note, there is an extension that provides URI datatype in PostgreSQL:
/// <https://github.com/petere/pguri>
pub fn parse_image_url(raw: Option<&str>) -> Result<Option<Url>, Error> {
    match raw {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            let url =
                Url::parse(s).map_err(|_| anyhow!("Failed to parse stored image path as URL"))?;
            Ok(Some(url))
        }
    }
}
