use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{data::dream_dto::DreamSummaryDto, error::error::ApiError, utils::datetime::{to_date, today, iter_between_two_dates}, model::summary_model::DreamSummary, repository::dream_repository::find_dreams_by_date};

use super::util_service::{check_days_out_of_bounds, dates_are_in_order};

pub async fn dream_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<DreamSummaryDto>, ApiError> {
    let date = to_date(string_date)?;
    let summary = get_dream_summary(baby_id, date).await?;
    Ok(Json(summary.into()))
}

pub async fn dream_summary_today_service(baby_id: i32) -> Result<Json<DreamSummaryDto>, ApiError> {
    let date = today();
    let summary = get_dream_summary(baby_id, date).await?;
    Ok(Json(summary.into()))
}

pub async fn dream_summary_range_service(
    baby_id: i32,
    from_date_string: &str,
    to_date_string: &str,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let from = to_date(from_date_string)?;
    let to = to_date(to_date_string)
        .unwrap_or(today())
        .checked_add_days(Days::new(1))
        .unwrap();
    dream_summary_range(baby_id, from, to).await
}
pub async fn dream_summary_last_days_service(
    baby_id: i32,
    last_days: u64,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    check_days_out_of_bounds(last_days)?;
    let today = today();
    let from_date = today.checked_sub_days(Days::new(last_days)).unwrap();
    dream_summary_range(baby_id, from_date, today).await
}

async fn dream_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    dates_are_in_order(from_date, to_date)?;
    let mut summary: Vec<DreamSummary> = Vec::new();
    let days_between = iter_between_two_dates(from_date, to_date);
    for day in days_between {
        let ds = get_dream_summary(baby_id, day).await?;
        if !ds.summary().is_zero() {
            summary.push(ds)
        }
    }

    Ok(into_json_summary(summary))
}

async fn get_dream_summary(baby_id: i32, day: NaiveDate) -> Result<DreamSummary, ApiError> {
    let dreams = find_dreams_by_date(baby_id, day).await?;
    let summary = DreamSummary::new(day, dreams);
    Ok(summary)
}

fn into_json_summary(summaries: Vec<DreamSummary>) -> Json<Vec<DreamSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}
