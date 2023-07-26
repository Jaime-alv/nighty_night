use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub enum Response {
    NewRecord,
    UpdateRecord,
    DeleteRecord,
    UserLogIn(String),
    NewUser(String),
    ActiveStatusUpdate,
}

impl Response {
    fn get_response(&self) -> (StatusCode, String) {
        match self {
            Response::NewRecord => (StatusCode::CREATED, "New record added.".to_string()),
            Response::UpdateRecord => (StatusCode::ACCEPTED, "Update record.".to_string()),
            Response::DeleteRecord => (StatusCode::ACCEPTED, "Delete record.".to_string()),
            Response::UserLogIn(username) => {
                (StatusCode::OK, format!("User logged in: {username}."))
            }
            Response::NewUser(username) => {
                (StatusCode::CREATED, format!("New user added: {username}."))
            }
            Response::ActiveStatusUpdate => {
                (StatusCode::ACCEPTED, "User status update.".to_string())
            }
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