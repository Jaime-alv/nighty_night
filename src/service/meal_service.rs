use crate::{
    data::meal_dto::{MealDto, MealSummary, NewMealDto},
    error::error::ApiError,
    mapping::meal_mapper::from_meal_to_meal_dto_vector,
    model::meals_model::InsertableMeal,
    repository::meal_repository::{find_meals_by_date, get_all_meals_from_baby, ingest_meal},
    utils::{
        datetime::{format_date, format_duration, now, to_date, to_date_time},
        response::Response,
    },
};

use super::response_service::ok;

pub async fn post_meal_service(new_meal: NewMealDto, baby_id: i32) -> Result<Response, ApiError> {
    let timestamp = match new_meal.date {
        Some(date) => to_date_time(&date),
        None => now(),
    };
    let timestamp_to_time = match new_meal.to_time {
        Some(date) => Some(to_date_time(&date)),
        None => None,
    };
    let meal = InsertableMeal::new(baby_id, timestamp, new_meal.quantity, timestamp_to_time);
    match ingest_meal(meal) {
        Ok(_) => Ok(ok("New meal added").await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn get_meals_service(baby_id: i32) -> Result<Vec<MealDto>, ApiError> {
    match get_all_meals_from_baby(baby_id) {
        Ok(meals) => Ok(from_meal_to_meal_dto_vector(meals).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn filter_meals_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Vec<MealDto>, ApiError> {
    let date = to_date(&string_date);
    match find_meals_by_date(baby_id, date) {
        Ok(meals) => Ok(from_meal_to_meal_dto_vector(meals).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn meal_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<MealSummary, ApiError> {
    let date = to_date(&string_date);
    let selected_meals = match find_meals_by_date(baby_id, date) {
        Ok(dreams) => dreams,
        Err(error) => return Err(ApiError::DBError(error)),
    };
    let size = selected_meals.len();
    let formula_feedings = selected_meals
        .iter()
        .map(|meal| meal.quantity())
        .reduce(|acc, feeds| acc + feeds)
        .unwrap();
    let duration = selected_meals
        .into_iter()
        .map(|meal| meal.elapsed())
        .reduce(|acc, e| acc.checked_add(&e).unwrap())
        .unwrap();
    Ok(MealSummary {
        date: format_date(date),
        total_feedings: size,
        nursing_time: format_duration(duration.num_minutes()),
        formula: formula_feedings,
    })
}
