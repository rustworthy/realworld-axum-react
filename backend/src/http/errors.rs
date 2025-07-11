use rocket::serde::{json::Json, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

/// Container with validation errors.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/error-handling/>
#[derive(Debug, ToSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Validation {
    pub errors: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Responder)]
pub(crate) enum Error {
    #[response(status = 422)]
    Validation(Json<Validation>),
}
