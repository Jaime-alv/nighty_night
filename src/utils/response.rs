use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub struct Response {
    status: StatusCode,
    message: String,
}

impl Response {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status;
        let code = status_code.as_u16();
        let body = Json(json!({ "code": code, "message": self.message }));

        (status_code, body).into_response()
    }
}
