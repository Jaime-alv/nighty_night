use crate::{
    data::meal_dto::{MealDto, MealSummary, NewMealDto},
    error::error::ApiError,
    mapping::meal_mapper::VecMeal,
    model::meals_model::InsertableMeal,
    repository::meal_repository::{find_meals_by_date, get_all_meals_from_baby, ingest_meal},
    utils::{
        datetime::{format_date, format_duration, now, to_date},
        response::Response,
    },
};

use super::util_service::{ok, uncover_date};

pub async fn post_meal_service(new_meal: NewMealDto, baby_id: i32) -> Result<Response, ApiError> {
    let timestamp = uncover_date(new_meal.date)?;
    let timestamp_to_time = uncover_date(new_meal.to_time)?;
    let meal = InsertableMeal::new(
        baby_id,
        timestamp.unwrap_or(now()),
        new_meal.quantity,
        timestamp_to_time,
    );
    ingest_meal(meal).await?;
    Ok(ok("New meal added"))
}

pub async fn get_meals_service(baby_id: i32) -> Result<Vec<MealDto>, ApiError> {
    let meals = get_all_meals_from_baby(baby_id).await?;
    Ok(VecMeal::new(meals).into())
}

pub async fn filter_meals_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Vec<MealDto>, ApiError> {
    let date = to_date(string_date)?;
    let meals = find_meals_by_date(baby_id, date).await?;
    Ok(VecMeal::new(meals).into())
}

pub async fn meal_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<MealSummary, ApiError> {
    let date = to_date(string_date)?;
    let selected_meals = find_meals_by_date(baby_id, date).await?;
    let size = selected_meals.len();
    let formula_feedings = selected_meals
        .iter()
        .map(|meal| meal.quantity())
        .reduce(|acc, feeds| acc + feeds)
        .unwrap();
    let sum_duration = selected_meals
        .into_iter()
        .map(|meal| meal.elapsed())
        .reduce(|acc, e| acc.checked_add(&e).unwrap())
        .unwrap();
    Ok(MealSummary {
        date: format_date(date),
        total_feedings: size,
        nursing_time: format_duration(sum_duration.num_minutes()),
        formula: formula_feedings,
    })
}
