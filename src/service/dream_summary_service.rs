use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::{dream_dto::DreamSummaryDto, query_dto::Pagination},
    error::error::ApiError,
    model::{dream_model::Dream, summary_model::DreamSummary},
    repository::dream_repository::{
        find_all_dreams_sorted, find_dreams_summary, obtain_first_and_last_dream_date,
    },
    utils::datetime::{iter_between_two_dates, now, today},
};

use super::util_service::{paginate_over_dates, records_is_not_empty};

pub async fn dream_summary_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let (start, stop) = paginate_over_dates(pagination, from_date, to_date);
    let summary = fetch_dream_summary_range(baby_id, start, stop).await?;
    Ok(into_json_summary(summary))
}

/// Need to add plus one day to look for certain date.
async fn fetch_dream_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<DreamSummary>, ApiError> {
    // dates_are_in_order(from_date, to_date)?;
    let plus_one = to_date.checked_add_days(Days::new(1)).unwrap();
    let mut summary_vec: Vec<DreamSummary> = Vec::new();
    let dreams = find_dreams_summary(baby_id, from_date, plus_one)?;
    let dates = iter_between_two_dates(from_date, plus_one);
    for day in dates {
        let partial_dreams = dreams
            .clone()
            .into_iter()
            .filter(|dream| dream.to_date().unwrap_or(now()).date().eq(&day))
            .collect::<Vec<Dream>>();
        if !partial_dreams.is_empty() {
            let summary = DreamSummary::new(day, partial_dreams);
            summary_vec.push(summary)
        }
    }
    records_is_not_empty(summary_vec)
}

pub async fn dream_summary_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let today = today();
    let from_date = today
        .checked_sub_days(Days::new(last_days.try_into().unwrap()))
        .unwrap();
    let (start, stop) = paginate_over_dates(pagination, from_date, today);
    let summaries = fetch_dream_summary_range(baby_id, start, stop).await?;
    Ok(into_json_summary(summaries))
}

fn into_json_summary(summaries: Vec<DreamSummary>) -> Json<Vec<DreamSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}

pub async fn get_all_dream_summaries_service(
    baby_id: i32,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let mut summaries: Vec<DreamSummary> = Vec::new();
    let all_records = find_all_dreams_sorted(baby_id)?;
    let initial_record = match all_records.first() {
        Some(initial) => initial.to_date().unwrap().date(),
        None => return Ok(into_json_summary(summaries)),
    };
    let today = today().checked_add_days(Days::new(1)).unwrap();
    let dates = iter_between_two_dates(initial_record, today);
    for day in dates {
        let partial_dreams = all_records
            .clone()
            .into_iter()
            .filter(|dream| dream.to_date().unwrap().date().eq(&day))
            .collect::<Vec<Dream>>();
        if !partial_dreams.is_empty() {
            let summary = DreamSummary::new(day, partial_dreams);
            summaries.push(summary)
        }
    }
    Ok(into_json_summary(summaries))
}

pub async fn get_all_summary_records_paginated(
    baby_id: i32,
    pagination: Pagination,
) -> Result<Json<Vec<DreamSummaryDto>>, ApiError> {
    let (raw_start, raw_stop) = obtain_first_and_last_dream_date(baby_id)?;
    let (start, stop) = paginate_over_dates(pagination, raw_start, raw_stop);
    let summary = fetch_dream_summary_range(baby_id, start, stop).await?;
    Ok(into_json_summary(summary))
}
