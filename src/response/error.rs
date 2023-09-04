use std::fmt::Display;

use axum::response::IntoResponse;
use diesel::result::Error;
use hyper::StatusCode;
use redis::RedisError;
use serde::Serialize;

use super::response_helper::display_as;

#[derive(Debug)]
pub enum ApiError {
    EmptyBody,
    IncorrectPassword,
    Forbidden,
    DuplicateUser,
    /// No user found
    NoUser,
    NoActiveUser,
    PageNotFound,
    NoRecord,
    LoginRequired,
    DatesUnordered,
    CastError(String),
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
            ApiError::NoUser => (StatusCode::NOT_FOUND, String::from("No user found.")),
            ApiError::DuplicateUser => (
                StatusCode::BAD_REQUEST,
                String::from("User already exists."),
            ),
            ApiError::Generic400Error(msg) => (StatusCode::BAD_REQUEST, String::from(msg)),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, String::from("Forbidden.")),
            ApiError::NoActiveUser => (
                StatusCode::UNAUTHORIZED,
                String::from("User is not active."),
            ),
            ApiError::PageNotFound => (
                StatusCode::NOT_FOUND,
                String::from("This is not the page you are looking for."),
            ),
            ApiError::LoginRequired => (StatusCode::UNAUTHORIZED, String::from("Login required.")),
            ApiError::DatesUnordered => (
                StatusCode::BAD_REQUEST,
                String::from("Target date must be higher."),
            ),
            ApiError::NoRecord => (StatusCode::NOT_FOUND, String::from("No record found.")),
            ApiError::CastError(msg) => (StatusCode::BAD_REQUEST, format!("Casting error: {msg}")),
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
        let error = ErrorField {
            status: status_code.as_u16(),
            detail: &msg,
            r#type: "error",
        };
        let body = display_as(error, None);

        (status_code, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorField<'a> {
    status: u16,
    detail: &'a str,
    r#type: &'a str
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (status_code, msg) = self.get_error();
        let readable_msg = format!("{status_code}: {msg}");
        write!(f, "{}", readable_msg)
    }
}
