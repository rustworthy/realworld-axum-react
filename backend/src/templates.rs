use askama::Template;
use url::Url;

#[derive(Template)]
#[template(path = "otp_email.html")]
pub struct OTPEmailHtml<'a> {
    pub otp_code: &'a str,
    pub app_url: &'a Url,
}

#[derive(Template)]
#[template(path = "otp_email.txt")]
pub struct OTPEmailText<'a> {
    pub otp_code: &'a str,
    pub app_url: &'a Url,
}
