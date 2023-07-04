use axum::Json;

use crate::{
    data::meal_dto::{MealDto, MealSummaryDto, NewMealDto},
    error::error::ApiError,
    model::{
        meals_model::{InsertableMeal, Meal},
        summary_model::MealSummary,
    },
    repository::meal_repository::{find_meals_by_date, get_all_meals_from_baby, ingest_meal},
    utils::{
        datetime::{now, to_date},
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

pub async fn get_meals_service(baby_id: i32) -> Result<Json<Vec<MealDto>>, ApiError> {
    let meals = get_all_meals_from_baby(baby_id).await?;
    Ok(into_json(meals))
}

pub async fn filter_meals_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<Vec<MealDto>>, ApiError> {
    let date = to_date(string_date)?;
    let meals = find_meals_by_date(baby_id, date).await?;
    Ok(into_json(meals))
}

pub async fn meal_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<MealSummaryDto>, ApiError> {
    let date = to_date(string_date)?;
    let meals = find_meals_by_date(baby_id, date).await?;
    let summary = MealSummary::new(date, meals);
    Ok(Json(summary.into()))
}

fn into_json(meals: Vec<Meal>) -> Json<Vec<MealDto>> {
    Json(meals.into_iter().map(|meal| meal.into()).collect())
}
