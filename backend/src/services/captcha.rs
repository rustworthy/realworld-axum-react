use secrecy::{ExposeSecret, SecretString};
use std::{collections::HashMap, time::Duration};

const CAPTCHA_VERIFY_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const CAPTCHA_VERIFY_ENDPOINT: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[allow(unused)]
pub struct Captcha {
    client: reqwest::Client,
    secret: SecretString,
}

/// Tursntile server response payload.
///
/// Decribe other props (will be an enum of successful and faield check) if needed
/// as per [docs](https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#api-response-format)
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationResult {
    pub success: bool,
}

impl Captcha {
    pub fn new(secret: SecretString, timeout: Option<Duration>) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout.unwrap_or(CAPTCHA_VERIFY_REQUEST_TIMEOUT))
            .build()
            .expect("all required args passed");
        Self {
            client: http_client,
            secret,
        }
    }

    pub async fn verify<S>(&self, token: S) -> anyhow::Result<VerificationResult>
    where
        S: AsRef<str>,
    {
        let mut form = HashMap::new();
        form.insert("secret", self.secret.expose_secret());
        form.insert("response", token.as_ref());
        let result = self
            .client
            .post(CAPTCHA_VERIFY_ENDPOINT)
            .form(&form)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
}
