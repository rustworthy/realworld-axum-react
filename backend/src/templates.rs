use askama::Template;

#[derive(Template)]
#[template(path = "otp_email.html")]
pub struct OTPEmailHtml<'a> {
    pub otp_code: &'a str,
}

#[derive(Template)]
#[template(path = "otp_email.txt")]
pub struct OTPEmailText<'a> {
    pub otp_code: &'a str,
}