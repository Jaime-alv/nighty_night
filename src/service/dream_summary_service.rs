use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::dream_dto::DreamSummaryDto,
    error::error::ApiError,
    model::{dream_model::Dream, summary_model::DreamSummary},
    repository::dream_repository::find_dreams_summary,
    utils::datetime::{iter_between_two_dates, to_date, today},
};

use super::util_service::{check_days_out_of_bounds, dates_are_in_order};

pub async fn dream_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<DreamSummaryDto>, ApiError> {
    let date = to_date(string_date)?;
    let record = one_day_summary(baby_id, date).await?;
    Ok(Json(record.into()))
}

pub async fn dream_summary_today_service(baby_id: i32) -> Result<Json<DreamSummaryDto>, ApiError> {
    let date = today();
    let record = one_day_summary(baby_id, date).await?;
    Ok(Json(record.into()))
}

pub async fn dream_summary_range_service(
    baby_id: i32,
    from_date_string: &str,
    to_date_string: &str,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let from = to_date(from_date_string)?;
    let to = to_date(to_date_string)?;
    let summary = fetch_dream_summary_range(baby_id, from, to).await?;
    Ok(into_json_summary(summary))
}

/// Need to add plus one day to look for certain date.
async fn one_day_summary(baby_id: i32, day: NaiveDate) -> Result<DreamSummary, ApiError> {
    let plus_one = day.checked_add_days(Days::new(1)).unwrap();
    let summary = fetch_dream_summary_range(baby_id, day, plus_one).await?;
    let record = obtain_first_record(summary, day)?;
    Ok(record)
}

async fn fetch_dream_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<DreamSummary>, ApiError> {
    dates_are_in_order(from_date, to_date)?;
    let mut summary_vec: Vec<DreamSummary> = Vec::new();
    let dreams = find_dreams_summary(baby_id, from_date, to_date).await?;
    let dates = iter_between_two_dates(from_date, to_date);
    for day in dates {
        let partial_dreams = dreams
            .clone()
            .into_iter()
            .filter(|dream| dream.to_date().date().eq(&day))
            .collect::<Vec<Dream>>();
        let summary = DreamSummary::new(day, partial_dreams);
        if !summary.summary().is_zero() {
            summary_vec.push(summary)
        }
    }
    Ok(summary_vec)
}

fn obtain_first_record(
    summaries: Vec<DreamSummary>,
    date: NaiveDate,
) -> Result<DreamSummary, ApiError> {
    match summaries.first() {
        Some(element) => Ok(element.clone()),
        None => Err(ApiError::NoRecord(date)),
    }
}

pub async fn dream_summary_last_days_service(
    baby_id: i32,
    last_days: u64,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    check_days_out_of_bounds(last_days)?;
    let today = today();
    let from_date = today.checked_sub_days(Days::new(last_days)).unwrap();
    let summaries = fetch_dream_summary_range(baby_id, from_date, today).await?;
    Ok(into_json_summary(summaries))
}

fn into_json_summary(summaries: Vec<DreamSummary>) -> Json<Vec<DreamSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}
