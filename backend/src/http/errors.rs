use axum::{Json, response::IntoResponse};
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
    Validation(Json<Validation>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
