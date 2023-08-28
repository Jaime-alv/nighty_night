use axum::response::IntoResponse;

use hyper::StatusCode;
use serde::Serialize;

use super::response_helper::display_as;

/// Return a factory set message.
pub enum MsgResponse {
    NewRecord,
    UpdateRecord,
    DeleteRecord,
    DeleteXRecords(usize),
    UserLogIn(String),
    NewUser(String),
    ActiveStatusUpdate,
    LogoutUser,
}

impl MsgResponse {
    fn get_response(&self) -> (StatusCode, String) {
        match self {
            MsgResponse::NewRecord => (StatusCode::CREATED, "New record added.".to_string()),
            MsgResponse::UpdateRecord => (StatusCode::ACCEPTED, "Update record.".to_string()),
            MsgResponse::DeleteRecord => (StatusCode::ACCEPTED, "Delete record.".to_string()),
            MsgResponse::UserLogIn(username) => {
                (StatusCode::OK, format!("User logged in: {username}."))
            }
            MsgResponse::NewUser(username) => {
                (StatusCode::CREATED, format!("New user added: {username}."))
            }
            MsgResponse::ActiveStatusUpdate => {
                (StatusCode::ACCEPTED, "User status update.".to_string())
            }
            MsgResponse::LogoutUser => (StatusCode::ACCEPTED, "User logged out".to_string()),
            MsgResponse::DeleteXRecords(number) => {
                (StatusCode::ACCEPTED, format!("{number} records deleted."))
            }
        }
    }
}

impl IntoResponse for MsgResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = self.get_response();
        let body = display_as(msg, None, status_code);

        (status_code, body).into_response()
    }
}

/// Return data with pagination info.
pub struct PagedResponse<T>
where
    T: Serialize,
{
    data: T,
    pager: PageInfo,
}

impl<T> PagedResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, current: i64, total_pages: i64) -> Self {
        let pager = PageInfo {
            current,
            total_pages,
        };
        Self { data, pager }
    }
}

#[derive(Serialize)]
pub struct PageInfo {
    current: i64,
    total_pages: i64,
}

impl<T> IntoResponse for PagedResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::OK;
        let body = display_as(self.data, Some(self.pager), status_code);

        (status_code, body).into_response()
    }
}

/// Return a single record.
pub struct RecordResponse<T>
where
    T: Serialize,
{
    data: T,
}

impl<T> RecordResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> IntoResponse for RecordResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::OK;
        let body = display_as(self.data, None, status_code);

        (status_code, body).into_response()
    }
}
