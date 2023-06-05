use crate::{
    data::meal_dto::{MealDto, NewMealDto},
    error::error::ApiError,
    mapping::meal_mapper::from_meal_to_meal_dto_vector,
    model::meals_model::InsertableMeal,
    repository::meal_repository::{get_all_meals_from_baby, ingest_meal},
    utils::{
        datetime::{now, to_date_time},
        response::Response,
    },
};

use super::response_service::ok;

pub async fn post_meal_service(new_meal: NewMealDto, baby_id: i32) -> Result<Response, ApiError> {
    let timestamp = match new_meal.date {
        Some(date) => to_date_time(&date),
        None => now(),
    };
    let meal = InsertableMeal::new(baby_id, timestamp, new_meal.quantity, new_meal.elapsed);
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
