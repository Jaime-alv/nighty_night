use hyper::StatusCode;

use crate::{error::error::ApiError, utils::response::Response};

pub fn forbidden() -> ApiError {
    ApiError::Forbidden
}

pub fn ok(msg: &str) -> Response {
    Response::new(StatusCode::OK, msg)
}

pub fn empty_query() -> ApiError {
    ApiError::EmptyQuery
}