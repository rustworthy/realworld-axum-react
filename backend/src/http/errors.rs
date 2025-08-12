use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use sqlx::error::DatabaseError;
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
    Sqlx(sqlx::Error),
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
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Sqlx(err)
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
            Error::Sqlx(e) => {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl Error {
    pub fn unprocessable_entity<K, V, D>(errors: D) -> Self
    where
        K: Into<String>,
        V: Into<String>,
        D: IntoIterator<Item = (K, V)>,
    {
        let mut error_map = BTreeMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into())
        }

        Self::Unprocessable(Validation { errors: error_map })
    }
}

pub(crate) trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
