use chrono::{Days, NaiveDate};

use crate::{
    data::{meal_dto::MealSummaryDto, query_dto::Pagination},
    model::{meals_model::Meal, summary_model::MealSummary},
    repository::meal_repository::{select_meals_by_date_range, select_date_first_and_last_meal},
    response::{error::ApiError, response::PagedResponse},
    utils::datetime::{iter_between_two_dates, today},
};

use super::util_service::{paginate_over_dates, round_total_pages};

pub async fn get_meals_summary_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealSummaryDto>>, ApiError> {
    let current = pagination.page();
    let total_pages = round_total_pages(from_date, to_date, pagination.per_page());
    let (start, stop) = paginate_over_dates(pagination, from_date, to_date);
    let summary = fetch_meal_summary_range(baby_id, start, stop).await?;
    let response = PagedResponse::new(into_summary_dto(summary), current, total_pages);
    Ok(response)
}

/// Need to add plus one day to look for certain date.
async fn fetch_meal_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<MealSummary>, ApiError> {
    let plus_one = to_date.checked_add_days(Days::new(1)).unwrap();
    let mut summary_vec: Vec<MealSummary> = Vec::new();
    let dreams = select_meals_by_date_range(baby_id, from_date, plus_one)?;
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

pub async fn get_meals_summary_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealSummaryDto>>, ApiError> {
    let today = today();
    let from_date = today
        .checked_sub_days(Days::new(last_days.try_into().unwrap()))
        .unwrap();
    get_meals_summary_range_service(baby_id, from_date, today, pagination).await
}

fn into_summary_dto(summaries: Vec<MealSummary>) -> Vec<MealSummaryDto> {
    summaries.into_iter().map(|item| item.into()).collect()
}

pub async fn get_meals_summary_all_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealSummaryDto>>, ApiError> {
    let current = pagination.page();
    let (raw_start, raw_stop) = select_date_first_and_last_meal(baby_id)?;
    let total_pages = round_total_pages(raw_start, raw_stop, pagination.per_page());
    let (start, stop) = paginate_over_dates(pagination, raw_start, raw_stop);
    let summary = fetch_meal_summary_range(baby_id, start, stop).await?;
    let response = PagedResponse::new(into_summary_dto(summary), current, total_pages);
    Ok(response)
}
