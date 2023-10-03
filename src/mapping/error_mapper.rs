use std::num::{ParseIntError, TryFromIntError};

use diesel::result::Error;
use redis::RedisError;

use crate::response::error::ApiError;

impl From<chrono::ParseError> for ApiError {
    fn from(value: chrono::ParseError) -> Self {
        ApiError::CastError(value.to_string())
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        match value.to_string().as_str() {
            "Record not found" => ApiError::NoRecordFound,
            _ => {
                let error = ApiError::DBError(value);
                tracing::error!("{error}");
                error
            }
        }
    }
}

impl From<RedisError> for ApiError {
    fn from(value: RedisError) -> Self {
        let error = ApiError::Redis(value);
        tracing::error!("{error}");
        error
    }
}

impl From<ApiError> for anyhow::Error {
    fn from(error: ApiError) -> Self {
        anyhow::anyhow!(error)
    }
}

impl From<ParseIntError> for ApiError {
    fn from(value: ParseIntError) -> Self {
        ApiError::CastError(value.to_string())
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(value: TryFromIntError) -> Self {
        ApiError::CastError(value.to_string())
    }
}

impl From<uuid::Error> for ApiError {
    fn from(value: uuid::Error) -> Self {
        ApiError::CastError(value.to_string())
    }
}
