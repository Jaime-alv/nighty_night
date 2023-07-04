use std::collections::HashMap;

use chrono::NaiveDateTime;
use hyper::StatusCode;

use crate::{error::error::ApiError, utils::{response::Response, datetime::to_date_time}};

pub fn ok(msg: &str) -> Response {
    Response::new(StatusCode::OK, msg)
}

pub fn not_found() -> ApiError {
    ApiError::NotFound
}

pub fn parse_query_field(value: HashMap<String, String>, field: &str) -> Result<String, ApiError> {
    match value.get(field) {
        Some(item) => Ok(item.to_string()),
        None => Err(ApiError::EmptyQuery),
    }
}

pub fn uncover_date(date: Option<String>) -> Result<Option<NaiveDateTime>, ApiError> {
    match date {
        Some(d) => match to_date_time(&d) {
            Ok(date) => Ok(Some(date)),
            Err(e) => Err(e.into()),
        },
        None => Ok(None),
    }
}