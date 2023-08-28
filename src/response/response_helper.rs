use axum::Json;
use hyper::StatusCode;
use serde::Serialize;
use serde_json::{Value, json};

use super::response::PageInfo;


/// How info will be displayed in json format.
pub(super) fn display_as<T>(data: T, pager: Option<PageInfo>, status_code: StatusCode) -> Json<Value>
where
    T: Serialize,
{
    let code = status_code.as_u16();
    let body = match pager {
        Some(pages) => json!({"code": code, "data": data, "page_info": pages}),
        None => json!({"code": code, "data": data}),
    };
    Json(body)
}
