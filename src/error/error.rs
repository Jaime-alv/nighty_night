use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub enum ApiError {
    EmptyBody,
    IncorrectPassword,
    DBError,
    DuplicateUser,
    NoUser,
    NoEntryFound,
    Generic50XError(&'static str),
    Generic403Error(&'static str),
}

impl ApiError {
    fn get_error<'a>(self) -> (StatusCode, &'a str) {
        match self {
            // 40X Error
            ApiError::EmptyBody => (StatusCode::BAD_REQUEST, "Empty body."),
            ApiError::IncorrectPassword => {
                (StatusCode::BAD_REQUEST, "Incorrect username or password.")
            }
            ApiError::NoUser => (StatusCode::BAD_REQUEST, "No user found."),
            ApiError::DuplicateUser => (StatusCode::BAD_REQUEST, "User already exists."),
            ApiError::Generic403Error(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NoEntryFound => (StatusCode::BAD_REQUEST, "No entry found."),

            // 50X Error
            ApiError::DBError => (StatusCode::INTERNAL_SERVER_ERROR, "Could not create entry."),
            ApiError::Generic50XError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
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

struct BodyError<'a> {
    code: u16,
    message: &'a str,
    sub_message: Option<&'a str>,
}

impl<'a> BodyError<'a> {
    fn new(code: u16, message: &'a str, sub_message: Option<&'a str>) -> Self {
        Self {
            code,
            message,
            sub_message,
        }
    }
}
