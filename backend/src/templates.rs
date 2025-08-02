use askama::Template;

#[derive(Template)]
#[template(path = "otp_email.html")]
pub struct OTPEmailHtml<'a> {
    pub otp_code: &'a str,
}

impl<'a> OTPEmailHtml<'a> {
    pub fn new(otp: &'a str) -> Self {
        Self { otp_code: otp }
    }
}

#[derive(Template)]
#[template(path = "otp_email.txt")]
pub struct OTPEmailText<'a> {
    pub otp_code: &'a str,
}

impl<'a> OTPEmailText<'a> {
    pub fn new(otp: &'a str) -> Self {
        Self { otp_code: otp }
    }
}
