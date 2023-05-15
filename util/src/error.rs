use tonic::Status;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database Error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Unable to connect to the cache. ")]
    RedisError(#[from] redis::RedisError),

    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0} Not Found")]
    NotFound(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}

impl From<ApiError> for Status {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::SqlxError(_) => Status::unavailable("database error"),
            ApiError::RedisError(_) => Status::unavailable("redis error"),
            ApiError::Internal(msg) => Status::internal(msg),
            ApiError::NotFound(msg) => Status::not_found(msg),
            ApiError::ValidationError(msg) => Status::invalid_argument(msg),
        }
    }
}