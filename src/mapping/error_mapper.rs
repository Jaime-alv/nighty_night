use std::num::ParseIntError;

use diesel::result::Error;
use redis::RedisError;

use crate::error::error::ApiError;

impl From<chrono::ParseError> for ApiError {
    fn from(value: chrono::ParseError) -> Self {
        ApiError::DateFormat(value)
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        let error = ApiError::DBError(value);
        tracing::error!("{error}");
        error
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
        ApiError::InvalidValue(value)
    }
}
