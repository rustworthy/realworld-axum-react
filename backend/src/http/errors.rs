use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use sqlx::error::DatabaseError;
use std::collections::BTreeMap;
use utoipa::ToSchema;
use validator::ValidationErrors;

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

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("unprocessable entity")]
    Unprocessable(Validation),

    #[error("unauthorized")]
    Unauthorized,

    #[error("internal error")]
    Internal(#[from] anyhow::Error),

    #[error("database driver error")]
    Sqlx(#[from] sqlx::Error),
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        let errors = BTreeMap::from([("body".to_string(), vec![value.to_string()])]);
        Self::Unprocessable(Validation { errors })
    }
}

impl From<ValidationErrors> for Error {
    fn from(errs: ValidationErrors) -> Self {
        let mapped = errs.field_errors().into_iter().map(|(field, errs)| {
            let msg = errs
                .first()
                .and_then(|e| e.message.clone())
                .unwrap_or_else(|| "invalid value".into());

            (field.to_string(), msg.to_string())
        });

        Error::unprocessable_entity(mapped)
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
            Self::Internal(e) => {
                error!(error = ?e, "innternal error occurred");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::Sqlx(e) => {
                error!(error = ?e, "database driver error");
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

// https://github.com/launchbadge/realworld-axum-sqlx/blob/f1b25654773228297e35c292f357d33b7121a101/src/http/error.rs#L173-L222
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
