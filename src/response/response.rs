use axum::{response::IntoResponse, Json};

use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

/// Return a factory set message.
pub enum MsgResponse {
    NewRecord,
    UpdateRecord,
    DeleteRecord,
    DeleteXRecords(usize),
    ActiveStatusUpdate,
    LogoutUser,
}

impl MsgResponse {
    fn get_response(&self) -> (StatusCode, String) {
        match self {
            MsgResponse::NewRecord => (StatusCode::CREATED, "New record added.".to_string()),
            MsgResponse::UpdateRecord => (StatusCode::ACCEPTED, "Update record.".to_string()),
            MsgResponse::DeleteRecord => (StatusCode::ACCEPTED, "Delete record.".to_string()),
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
        let message = Message {
            status: status_code.as_u16(),
            detail: &msg,
            r#type: "message",
            title: status_code.canonical_reason().unwrap(),
        };
        let body = Json(json!({"message": message}));

        (status_code, body).into_response()
    }
}

#[derive(Serialize)]
struct Message<'a> {
    status: u16,
    title: &'a str,
    r#type: &'a str,
    detail: &'a str,
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
        let pager = PageInfo::new(current, total_pages);
        Self { data, pager }
    }
}

#[derive(Serialize)]
pub struct PageInfo {
    current: i64,
    first: i64,
    prev: Option<i64>,
    next: Option<i64>,
    last: i64,
}

impl PageInfo {
    pub fn new(current: i64, total_pages: i64) -> Self {
        Self {
            current,
            first: 1,
            prev: if current.gt(&1) {
                Some(current - 1)
            } else {
                None
            },
            next: if current.lt(&total_pages) {
                Some(current + 1)
            } else {
                None
            },
            last: total_pages,
        }
    }
}

impl<T> IntoResponse for PagedResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::OK;
        let body = Json(json!({"data": self.data, "page_info": self.pager}));

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
        let body = Json(json!({ "data": self.data }));

        (status_code, body).into_response()
    }
}
