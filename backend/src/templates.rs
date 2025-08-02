use askama::Template;
use url::Url;

/// HTML template for email confirmation letter.
/// 
/// see <https://github.com/leemunroe/responsive-html-email-template>
#[derive(Template)]
#[template(path = "email_otp.html")]
pub struct OTPEmailHtml<'a> {
    pub otp_code: &'a str,
    pub app_url: &'a Url,
}

/// Text companion for email confirmation letter.
#[derive(Template)]
#[template(path = "email_otp.txt")]
pub struct OTPEmailText<'a> {
    pub otp_code: &'a str,
    pub app_url: &'a Url,
}
