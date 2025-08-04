use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use std::collections::BTreeMap;
use utoipa::ToSchema;

/// Container with validation errors.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/error-handling/>
#[derive(Debug, ToSchema, Serialize)]
pub(crate) struct Validation {
    #[schema(
        example = json!(
            BTreeMap::from([("body".to_string(), vec!["can't be empty".to_string()])])
        )
    )]
    pub errors: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
pub(crate) enum Error {
    Unprocessable(Validation),
    Unauthorized,
    Internal(String),
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        let errors = BTreeMap::from([("body".to_string(), vec![value.to_string()])]);
        Self::Unprocessable(Validation { errors })
    }
}
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                [(header::WWW_AUTHENTICATE, "Bearer")],
            )
                .into_response(),
            Self::Unprocessable(validation) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(validation)).into_response()
            }
            Self::Internal(reason) => {
                error!(reason);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
