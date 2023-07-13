use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub enum Response {
    NewRecord,
    UpdateRecord,
    DeleteRecord,
}

impl Response {
    fn get_response(&self) -> (StatusCode, &str) {
        match self {
            Response::NewRecord => (StatusCode::CREATED, "New record added."),
            Response::UpdateRecord => (StatusCode::ACCEPTED, "Update record."),
            Response::DeleteRecord => (StatusCode::ACCEPTED, "Delete record."),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = self.get_response();
        let code = status_code.as_u16();
        let body = Json(json!({ "code": code, "message": msg }));

        (status_code, body).into_response()
    }
}
