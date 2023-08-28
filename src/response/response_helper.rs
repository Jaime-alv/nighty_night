use axum::Json;

use serde::Serialize;
use serde_json::{json, Value};

use super::response::PageInfo;

/// How info will be displayed in json format.
pub(super) fn display_as<T>(data: T, pager: Option<PageInfo>) -> Json<Value>
where
    T: Serialize,
{
    let body = match pager {
        Some(pages) => json!({"data": data, "page_info": pages}),
        None => json!({ "data": data }),
    };
    Json(body)
}
