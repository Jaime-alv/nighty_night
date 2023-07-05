use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::meal_dto::MealSummaryDto,
    error::error::ApiError,
    model::summary_model::MealSummary,
    repository::meal_repository::find_meals_by_date,
    utils::datetime::{iter_between_two_dates, to_date, today},
};

use super::util_service::{check_days_out_of_bounds, dates_are_in_order};

pub async fn meal_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<MealSummaryDto>, ApiError> {
    let date = to_date(string_date)?;
    let summary = get_meal_summary(baby_id, date).await?;
    Ok(Json(summary.into()))
}

pub async fn meal_summary_today_service(baby_id: i32) -> Result<Json<MealSummaryDto>, ApiError> {
    let date = today();
    let summary = get_meal_summary(baby_id, date).await?;
    Ok(Json(summary.into()))
}

pub async fn meal_summary_range_service(
    baby_id: i32,
    from_date_string: &str,
    to_date_string: &str,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    let from = to_date(from_date_string)?;
    let to = to_date(to_date_string)
        .unwrap_or(today())
        .checked_add_days(Days::new(1))
        .unwrap();    
    meal_summary_range(baby_id, from, to).await
}
pub async fn meal_summary_last_days_service(
    baby_id: i32,
    last_days: u64,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    check_days_out_of_bounds(last_days)?;
    let today = today();
    let from_date = today.checked_sub_days(Days::new(last_days)).unwrap();
    meal_summary_range(baby_id, from_date, today).await
}

async fn meal_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Json<Vec<MealSummaryDto>>, ApiError> {
    dates_are_in_order(from_date, to_date)?;
    let mut summary: Vec<MealSummary> = Vec::new();
    let days_between = iter_between_two_dates(from_date, to_date);
    for day in days_between {
        let meal_summary = get_meal_summary(baby_id, day).await?;
        if meal_summary.total_feedings().ne(&0) {
            summary.push(meal_summary)
        }
    }

    Ok(into_json_summary(summary))
}

async fn get_meal_summary(baby_id: i32, day: NaiveDate) -> Result<MealSummary, ApiError> {
    let meals = find_meals_by_date(baby_id, day).await?;
    let summary = MealSummary::new(day, meals);
    Ok(summary)
}

fn into_json_summary(summaries: Vec<MealSummary>) -> Json<Vec<MealSummaryDto>> {
    Json(summaries.into_iter().map(|item| item.into()).collect())
}
