#![allow(unused)]

use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

#[async_trait::async_trait]
pub(crate) trait Mailer {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> anyhow::Result<()>;
}

const SEND_EMAIL_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct ResendMailer {
    sender: String,
    client: Resend,
}

impl ResendMailer {
    pub fn new(sender: String, token: &str, timeout: Option<Duration>) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout.unwrap_or(SEND_EMAIL_REQUEST_TIMEOUT))
            .build()
            .expect("all required args passed");
        let client = Resend::with_client(token, http_client);
        Self { client, sender }
    }

    #[tracing::instrument(name = "SEND EMAIL", fields(email_id), skip(html, text))]
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> Result<(), anyhow::Error> {
        let email = CreateEmailBaseOptions::new(&*self.sender, [to.to_string()], subject)
            .with_html(html)
            .with_text(text);
        let resp = self.client.emails.send(email).await?;
        tracing::info!(email_id = resp.id.to_string());
        Ok(())
    }
}

#[async_trait::async_trait]
impl Mailer for ResendMailer {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> anyhow::Result<()> {
        ResendMailer::send_email(self, to, subject, html, text).await
    }
}

pub(crate) struct StdoutMailer {
    sender: String,
}

impl StdoutMailer {
    pub fn new(sender: String) -> Self {
        Self { sender }
    }

    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl Mailer for StdoutMailer {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> anyhow::Result<()> {
        StdoutMailer::send_email(self, to, subject, html, text).await
    }
}

#[cfg(test)]
mod test {
    use super::ResendMailer;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use std::time::Duration;
    use wiremock::{
        Match, Mock, MockServer, Request, ResponseTemplate,
        matchers::{header, header_exists, method, path},
    };

    pub struct BodyMatcher<'a> {
        keys: &'a [&'a str],
    }

    impl Match for BodyMatcher<'_> {
        fn matches(&self, request: &Request) -> bool {
            let parsed: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if parsed.is_err() {
                return false;
            }
            let body = parsed.unwrap();
            for key in self.keys {
                if body.get(key).is_none() {
                    return false;
                };
            }
            true
        }
    }

    impl<'a> BodyMatcher<'a> {
        pub fn new(keys: &'a [&'a str]) -> Self {
            Self { keys }
        }
    }

    fn email_addr() -> String {
        SafeEmail().fake::<String>()
    }

    fn mailer(uri: String) -> ResendMailer {
        // Unfortunately, `resend-rs` crate does not expose a method  to set a
        // non-standard base URL and instead want to go via environment:
        // https://github.com/resend/resend-rust/blob/8fefbf5b1c45c68058974861bdb317c929207b5b/src/config.rs#L40-L48
        //
        // Imaginary API could look like:
        // ```rs
        // let http_client = reqwest::Client::builder()
        //      .timeout(Duration::from_secs(10))
        //      .build()
        //      .unwrap();
        // let resend_config = resend_rs::Config::builder('re_key')
        //      .base_url('http://wiremock:35353'.parse().unwrap())
        //      .client(http_client)
        //      .build();
        // let resend_client = resend_rs::Resend::with_config(resend_config);
        // ```
        // Until then, let's just set the environment variable:
        unsafe { std::env::set_var("RESEND_BASE_URL", uri) };
        ResendMailer::new(
            "test@domain.io".to_string(),
            "re_secret",
            Some(Duration::from_millis(500)),
        )
    }

    fn subject() -> String {
        Sentence(0..1).fake()
    }

    fn text() -> String {
        Paragraph(0..2).fake()
    }

    #[tokio::test]
    async fn send_email_processes_body_correctly() {
        let mock_server = MockServer::start().await;

        Mock::given(path("/emails"))
            .and(method("POST"))
            .and(header_exists("Authorization"))
            .and(header("content-type", "application/json"))
            .and(BodyMatcher::new(&["from", "to", "subject", "text"]))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mailer(mock_server.uri())
            .send_email(&email_addr(), &subject(), &text(), &text())
            .await;
    }
}
