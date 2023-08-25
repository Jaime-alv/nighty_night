use axum::{response::IntoResponse, Json};

use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
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
    pub fn new(data: T, pager: PageInfo) -> Self {
        Self { data, pager }
    }
}

#[derive(Serialize)]
pub struct PageInfo {
    current: i64,
    total_pages: i64,
}

impl PageInfo {
    pub fn new<T>(current: T, total_pages: T) -> Self
    where
        T: Into<i64>,
    {
        Self {
            current: current.into(),
            total_pages: total_pages.into(),
        }
    }
}

impl<T> IntoResponse for PagedResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(json!({"data": self.data, "page_info": self.pager})).into_response()
    }
}
