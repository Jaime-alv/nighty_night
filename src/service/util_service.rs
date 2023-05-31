use crate::error::error::ApiError;

pub fn forbidden() -> ApiError {
    ApiError::Forbidden
}