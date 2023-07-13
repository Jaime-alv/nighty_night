use axum::Json;
use chrono::NaiveDate;

use crate::{
    data::meal_dto::{MealDto, NewMealDto, UpdateMealDto},
    error::error::ApiError,
    model::meals_model::{InsertableMeal, Meal},
    repository::meal_repository::{
        find_meal_by_id, find_meals_by_date, get_all_meals_from_baby, ingest_meal,
        patch_meal_record,
    },
    utils::{
        datetime::{convert_to_date_time, now},
        response::Response,
    },
};

use super::util_service::{date_time_are_in_order, uncover_date};

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
    Ok(Response::NewRecord)
}

pub async fn patch_meal_service(meal: UpdateMealDto, record: i32) -> Result<Response, ApiError> {
    let old_meal = find_meal_by_id(record).await?;
    let new_date = match meal.date {
        Some(v) => convert_to_date_time(&v)?,
        None => old_meal.date(),
    };
    let new_quantity = match meal.quantity {
        Some(v) => {
            if v.eq(&0) {
                None
            } else {
                Some(v)
            }
        }
        None => old_meal.quantity(),
    };
    let new_to_time = match meal.to_time {
        Some(v) => {
            let date_time = convert_to_date_time(&v)?;
            date_time_are_in_order(new_date.clone(), date_time)?;
            Some(date_time)
        }
        None => old_meal.to_time(),
    };
    let update_meal = Meal::new(
        record,
        old_meal.baby_id(),
        new_date,
        new_quantity,
        new_to_time,
    );
    patch_meal_record(update_meal).await?;
    Ok(Response::UpdateRecord)
}

pub async fn get_meals_service(baby_id: i32) -> Result<Json<Vec<MealDto>>, ApiError> {
    let meals = get_all_meals_from_baby(baby_id).await?;
    Ok(into_json(meals))
}

pub async fn filter_meals_by_date_service(
    baby_id: i32,
    date: NaiveDate,
) -> Result<Json<Vec<MealDto>>, ApiError> {
    let meals = find_meals_by_date(baby_id, date).await?;
    Ok(into_json(meals))
}

fn into_json(meals: Vec<Meal>) -> Json<Vec<MealDto>> {
    Json(meals.into_iter().map(|meal| meal.into()).collect())
}
