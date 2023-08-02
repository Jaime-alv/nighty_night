use axum::Json;
use serde::Serialize;

use crate::{
    error::error::ApiError,
    repository::admin_repository::{count_records, StatsDB},
};

#[derive(Serialize)]
pub struct RecordStats {
    table_name: &'static str,
    records: i64,
}

pub async fn show_stats_service() -> Result<Json<StatsDB<'static>>, ApiError> {
    let count = count_records()?;
    Ok(Json(count))
}
