use std::{fmt::Display, num::ParseIntError};

use axum::{response::IntoResponse, Json};
use chrono::NaiveDate;
use diesel::result::Error;
use hyper::StatusCode;
use redis::RedisError;
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    EmptyBody,
    IncorrectPassword,
    Forbidden,
    DuplicateUser,
    NoUser,
    NoEntryFound,
    NoActiveUser,
    EmptyQuery,
    NotFound,
    NoRecord(NaiveDate),
    LoginRequired,
    DatesUnordered,
    OutOfBounds(i16, i16),
    InvalidValue(ParseIntError),
    DateFormat(chrono::ParseError),
    DBError(Error),
    Redis(RedisError),
    Generic500Error(String),
    Generic400Error(String),
}

impl ApiError {
    fn get_error<'a>(&self) -> (StatusCode, String) {
        match self {
            // 40X Error
            ApiError::EmptyBody => (StatusCode::BAD_REQUEST, String::from("Empty body.")),
            ApiError::IncorrectPassword => (
                StatusCode::BAD_REQUEST,
                String::from("Incorrect username or password."),
            ),
            ApiError::NoUser => (StatusCode::BAD_REQUEST, String::from("No user found.")),
            ApiError::DuplicateUser => (
                StatusCode::BAD_REQUEST,
                String::from("User already exists."),
            ),
            ApiError::Generic400Error(msg) => (StatusCode::BAD_REQUEST, String::from(msg)),
            ApiError::NoEntryFound => (StatusCode::BAD_REQUEST, String::from("No entry found.")),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, String::from("Forbidden.")),
            ApiError::NoActiveUser => (
                StatusCode::UNAUTHORIZED,
                String::from("User is not active."),
            ),
            ApiError::EmptyQuery => (
                StatusCode::BAD_REQUEST,
                String::from("Query option required."),
            ),
            ApiError::DateFormat(msg) => (StatusCode::BAD_REQUEST, String::from(msg.to_string())),
            ApiError::NotFound => (StatusCode::NOT_FOUND, String::from("This is not the page you are looking for.")),
            ApiError::LoginRequired => (StatusCode::UNAUTHORIZED, String::from("Login required.")),
            ApiError::InvalidValue(value) => (StatusCode::BAD_REQUEST, format!("{value}")),
            ApiError::OutOfBounds(min, max) => (StatusCode::BAD_REQUEST, format!("Out of bounds: range between {min} and {max}.")),
            ApiError::DatesUnordered => (StatusCode::BAD_REQUEST, String::from("Target date must be higher.")),
            ApiError::NoRecord(date) => (StatusCode::BAD_REQUEST, format!("No record found with date: {date}")),
            // 50X Error
            ApiError::DBError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            ApiError::Generic500Error(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, String::from(msg))
            }
            ApiError::Redis(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Redis error: {error}"),
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = self.get_error();
        let code = status_code.as_u16();
        let body = Json(json!({ "code": code, "message": msg }));

        (status_code, body).into_response()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (status_code, msg) = self.get_error();
        let readable_msg = format!("{status_code}: {msg}");
        write!(f, "{}", readable_msg)
    }
}

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