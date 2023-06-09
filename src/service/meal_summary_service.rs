use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::meal_dto::MealSummaryDto,
    error::error::ApiError,
    model::{meals_model::Meal, summary_model::MealSummary},
    repository::meal_repository::{find_meals_by_date_range, find_all_meals_sorted},
    utils::datetime::{iter_between_two_dates, today},
};

use super::util_service::{check_days_out_of_bounds, dates_are_in_order};

pub async fn meal_summary_service(
    baby_id: i32,
    date: NaiveDate,
) -> Result<Json<MealSummaryDto>, ApiError> {
    let record = one_day_summary(baby_id, date).await?;
    Ok(Json(record.into()))
}

pub async fn meal_summary_today_service(baby_id: i32) -> Result<Json<MealSummaryDto>, ApiError> {
    let date = today();
    let record = one_day_summary(baby_id, date).await?;
    Ok(Json(record.into()))
}

pub async fn meal_summary_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let summary = fetch_meal_summary_range(baby_id, from_date, to_date).await?;
    Ok(into_json_summary(summary))
}

async fn one_day_summary(baby_id: i32, day: NaiveDate) -> Result<MealSummary, ApiError> {
    let summary = fetch_meal_summary_range(baby_id, day, day).await?;
    let record = obtain_first_record(summary, day)?;
    Ok(record)
}

/// Need to add plus one day to look for certain date.
async fn fetch_meal_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<MealSummary>, ApiError> {
    dates_are_in_order(from_date, to_date)?;
    let plus_one = to_date.checked_add_days(Days::new(1)).unwrap();
    let mut summary_vec: Vec<MealSummary> = Vec::new();
    let dreams = find_meals_by_date_range(baby_id, from_date, plus_one).await?;
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
    Ok(summary_vec)
}

fn obtain_first_record(
    summaries: Vec<MealSummary>,
    date: NaiveDate,
) -> Result<MealSummary, ApiError> {
    match summaries.first() {
        Some(element) => Ok(element.clone()),
        None => Err(ApiError::NoRecord(date)),
    }
}

pub async fn meal_summary_last_days_service(
    baby_id: i32,
    last_days: u64,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    check_days_out_of_bounds(last_days)?;
    let today = today();
    let from_date = today.checked_sub_days(Days::new(last_days)).unwrap();
    let summaries = fetch_meal_summary_range(baby_id, from_date, today).await?;
    Ok(into_json_summary(summaries))
}

fn into_json_summary(summaries: Vec<MealSummary>) -> Json<Vec<MealSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}

pub async fn get_all_meals_summaries_service(
    baby_id: i32,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let mut summaries: Vec<MealSummary> = Vec::new();
    let all_records = find_all_meals_sorted(baby_id).await?;
    let initial_record = match all_records.first() {
        Some(initial) => initial.date().date(),
        None => return Ok(into_json_summary(summaries)),
    };
    let today = today().checked_add_days(Days::new(1)).unwrap();
    let dates = iter_between_two_dates(initial_record, today);
    for day in dates {
        let partial_dreams = all_records
            .clone()
            .into_iter()
            .filter(|meal| meal.date().date().eq(&day))
            .collect::<Vec<Meal>>();
        if !partial_dreams.is_empty() {
            let summary = MealSummary::new(day, partial_dreams);
            summaries.push(summary)
        }
    }
    Ok(into_json_summary(summaries))
}