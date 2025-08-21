use crate::config::MailerTransport;
use resend_rs::types::{CreateEmailBaseOptions, EmailId};
use resend_rs::{Config, Resend, Result};
use std::time::Duration;
use url::Url;

const SEND_EMAIL_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct ResendMailer {
    sender: String,
    client: Resend,
    #[allow(unused)]
    transport: MailerTransport,
}

impl ResendMailer {
    pub fn new(
        sender: String,
        token: &str,
        base_url: Url,
        transport: MailerTransport,
        timeout: Option<Duration>,
    ) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout.unwrap_or(SEND_EMAIL_REQUEST_TIMEOUT))
            .build()
            .expect("all required args passed");
        let client = Resend::with_config(
            Config::builder(token)
                .client(http_client)
                .base_url(base_url)
                .build(),
        );
        Self {
            client,
            sender,
            transport,
        }
    }

    #[instrument(name = "SEND EMAIL", fields(email_id), skip(html, text))]
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> Result<EmailId, anyhow::Error> {
        let email = CreateEmailBaseOptions::new(&*self.sender, [to], subject)
            .with_html(html)
            .with_text(text);

        #[cfg(debug_assertions)]
        if let MailerTransport::Stdout = self.transport {
            use crate::utils::gen_alphanum_string;

            dbg!(&email);
            return Ok(EmailId::new(&gen_alphanum_string(8)));
        }

        let resp = self.client.emails.send(email).await?;
        tracing::info!(email_id = resp.id.to_string());
        Ok(resp.id)
    }
}

#[cfg(test)]
mod test {
    use crate::config::MailerTransport;

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
        ResendMailer::new(
            "test@domain.io".to_string(),     // "from" address
            "re_secret",                      // API token
            uri.parse().unwrap(),             // base url override
            MailerTransport::Http,            // transport
            Some(Duration::from_millis(500)), // request timeout
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
