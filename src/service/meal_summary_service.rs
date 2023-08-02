use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::{meal_dto::MealSummaryDto, query_dto::Pagination},
    error::error::ApiError,
    model::{meals_model::Meal, summary_model::MealSummary},
    repository::meal_repository::{find_meals_by_date_range, obtain_first_and_last_meal_date},
    utils::datetime::{iter_between_two_dates, today},
};

use super::util_service::{paginate_over_dates, records_is_not_empty};

pub async fn meal_summary_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let (start, stop) = paginate_over_dates(pagination, from_date, to_date);
    let summary = fetch_meal_summary_range(baby_id, start, stop).await?;
    Ok(into_json_summary(summary))
}

/// Need to add plus one day to look for certain date.
async fn fetch_meal_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<MealSummary>, ApiError> {
    let plus_one = to_date.checked_add_days(Days::new(1)).unwrap();
    let mut summary_vec: Vec<MealSummary> = Vec::new();
    let dreams = find_meals_by_date_range(baby_id, from_date, plus_one)?;
    let dates = iter_between_two_dates(from_date, plus_one);
    for day in dates {
        let partial_meals = dreams
            .clone()
            .into_iter()
            .filter(|meal| meal.date().date().eq(&day))
            .collect::<Vec<Meal>>();
        if !partial_meals.is_empty() {
            let summary = MealSummary::new(day, partial_meals);
            summary_vec.push(summary)
        }
    }
    records_is_not_empty(summary_vec)
}

pub async fn meal_summary_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let today = today();
    let from_date = today
        .checked_sub_days(Days::new(last_days.try_into().unwrap()))
        .unwrap();
    let (start, stop) = paginate_over_dates(pagination, from_date, today);
    let summaries = fetch_meal_summary_range(baby_id, start, stop).await?;

    Ok(into_json_summary(summaries))
}

fn into_json_summary(summaries: Vec<MealSummary>) -> Json<Vec<MealSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}

pub async fn get_all_summary_records_paginated(
    baby_id: i32,
    pagination: Pagination,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let (raw_start, raw_stop) = obtain_first_and_last_meal_date(baby_id)?;
    let (start, stop) = paginate_over_dates(pagination, raw_start, raw_stop);
    let summary = fetch_meal_summary_range(baby_id, start, stop).await?;
    Ok(into_json_summary(summary))
}
