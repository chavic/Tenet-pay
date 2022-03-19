use config::ConfigError;
use juniper::graphql_value;
use std::convert::From;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError(String),

    #[error("BadRequest: {0}")]
    BadRequest(String),
}

impl juniper::IntoFieldError for ServiceError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ServiceError::BadRequest(s) => juniper::FieldError::new(
                s,
                graphql_value!({
                    "type": "BAD_REQUEST"
                }),
            ),
            ServiceError::InternalServerError(s) => juniper::FieldError::new(
                &format!("Internal Error: {}", s),
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<uuid::Error> for ServiceError {
    fn from(_: uuid::Error) -> Self {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<ConfigError> for ServiceError {
    fn from(error: ConfigError) -> Self {
        ServiceError::InternalServerError(format!("{}", error))
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
