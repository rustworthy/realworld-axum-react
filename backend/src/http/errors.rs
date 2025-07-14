use rocket::serde::json;
use rocket::serde::{Serialize, json::Json};
use std::collections::BTreeMap;
use utoipa::ToSchema;

/// Container with validation errors.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/error-handling/>
#[derive(Debug, ToSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Validation {
    #[schema(
        example = json!(
            BTreeMap::from([("body".to_string(), vec!["can't be empty".to_string()])])
        )
    )]
    pub errors: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Responder)]
pub(crate) enum Error {
    #[allow(unused)]
    #[response(status = 422)]
    Validation(Json<Validation>),
}

impl<'a> From<json::Error<'a>> for Error {
    fn from(value: json::Error<'a>) -> Self {
        let mut errors = BTreeMap::new();
        errors.insert("body".into(), vec![value.to_string()]);
        Self::Validation(Json(Validation { errors }))
    }
}
