use axum::Json;
use serde::Serialize;

use crate::{
    error::error::ApiError,
    repository::{
        baby_repository::count_babies, dream_repository::count_dreams,
        meal_repository::count_meals, user_repository::count_users,
        weight_repository::count_weights,
    },
};

#[derive(Serialize)]
pub struct RecordStats {
    table_name: &'static str,
    records: i64,
}

pub async fn show_stats_service() -> Result<Json<Vec<RecordStats>>, ApiError> {
    let mut stats = Vec::<RecordStats>::with_capacity(5);
    let user_count = RecordStats {
        table_name: "users",
        records: count_users()?,
    };
    stats.push(user_count);
    let baby_count = RecordStats {
        table_name: "babies",
        records: count_babies()?,
    };
    stats.push(baby_count);
    let meal_count = RecordStats {
        table_name: "meals",
        records: count_meals()?,
    };
    stats.push(meal_count);
    let dream_count = RecordStats {
        table_name: "dreams",
        records: count_dreams()?,
    };
    stats.push(dream_count);
    let weight_count = RecordStats {
        table_name: "weights",
        records: count_weights()?,
    };
    stats.push(weight_count);
    Ok(Json(stats))
}
