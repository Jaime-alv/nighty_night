use axum::{response::IntoResponse, Json};

use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;

/// Return a factory set message.
#[derive(Debug)]
pub enum MsgResponse {
    UpdateRecord,
    DeleteRecord,
    DeleteXRecords(usize),
    ActiveStatusUpdate,
    LogoutUser,
}

impl MsgResponse {
    pub fn get_response(&self) -> (StatusCode, String) {
        match self {
            MsgResponse::UpdateRecord => (StatusCode::OK, "Update record.".to_string()),
            MsgResponse::DeleteRecord => (StatusCode::OK, "Delete record.".to_string()),
            MsgResponse::ActiveStatusUpdate => (StatusCode::OK, "User status update.".to_string()),
            MsgResponse::LogoutUser => (StatusCode::OK, "User logged out".to_string()),
            MsgResponse::DeleteXRecords(number) => {
                (StatusCode::OK, format!("{number} records deleted."))
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
            last: if total_pages.ge(&1) { total_pages } else { 1 },
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

#[derive(Debug)]
/// Return a single record.
pub struct RecordResponse<T>
where
    T: Serialize,
{
    pub data: T,
    pub status_code: u16,
}

impl<T> RecordResponse<T>
where
    T: Serialize,
{
    /// Get a formatted record from Database
    /// 
    /// Response returns 200
    pub fn new(data: T) -> Self {
        Self {
            data,
            status_code: 200,
        }
    }

    /// Method for adding entities into Database
    /// 
    /// Response returns 201
    pub fn new_entry(data: T) -> Self {
        Self {
            data,
            status_code: 201,
        }
    }
}

impl<T> IntoResponse for RecordResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or_default();
        let body = Json(json!({ "data": self.data }));

        (status_code, body).into_response()
    }
}
