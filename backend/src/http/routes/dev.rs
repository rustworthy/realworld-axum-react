use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Response},
};
use url::Url;

use crate::{
    state::AppContext,
    templates::{OTPEmailHtml, OTPEmailText},
    utils::gen_alphanum_string,
};

/// GET /dev/emails/otp_email.html?otp_code=abcd1234&app_url=http://localhost:8005/
pub async fn preview_otp_email(
    Query(params): Query<HashMap<String, String>>,
    State(ctx): State<Arc<AppContext>>,
) -> Response {
    let otp_code = params
        .get("otp_code")
        .map(|value| value.to_owned())
        .unwrap_or_else(|| gen_alphanum_string(8));
    let app_url: Url = params
        .get("app_url")
        .map(|value| value.to_owned())
        .map(|value| value.parse().ok())
        .unwrap_or(Some(ctx.frontend_url.clone()))
        .expect("Valid URL");
    match params.get("text") {
        Some(val) if val == "true" => {
            let content = OTPEmailText {
                otp_code: &otp_code,
                app_url: &app_url,
            }
            .render()
            .unwrap();
            content.into_response()
        }
        _ => Html(
            OTPEmailHtml {
                otp_code: &otp_code,
                app_url: &app_url,
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    }
}
