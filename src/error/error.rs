use std::fmt::Display;

use axum::{response::IntoResponse, Json};
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
            ApiError::IncorrectPassword => {
                (StatusCode::BAD_REQUEST, String::from("Incorrect username or password."))
            }
            ApiError::NoUser => (StatusCode::BAD_REQUEST, String::from("No user found.")),
            ApiError::DuplicateUser => (StatusCode::BAD_REQUEST, String::from("User already exists.")),
            ApiError::Generic400Error(msg) => (StatusCode::BAD_REQUEST, String::from(msg)),
            ApiError::NoEntryFound => (StatusCode::BAD_REQUEST, String::from("No entry found.")),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, String::from("Forbidden.")),
            ApiError::NoActiveUser => (StatusCode::UNAUTHORIZED, String::from("User is not active.")),

            // 50X Error
            ApiError::DBError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            ApiError::Generic500Error(msg) => (StatusCode::INTERNAL_SERVER_ERROR, String::from(msg)),
            ApiError::Redis(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis error: {error}")),
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
