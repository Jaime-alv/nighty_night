use chrono::{Days, NaiveDate};

use crate::{
    data::{
        meal_dto::{InputMealDto, MealDto, UpdateMeal},
        query_dto::Pagination,
    },
    model::meals_model::{InsertableMeal, Meal},
    repository::meal_repository::{
        delete_meal_from_db, find_meal_by_id, get_all_meals_from_baby, ingest_meal,
        meals_paginated_from_db, patch_meal_record,
    },
    response::{
        data_response::{PageInfo, PagedResponse},
        error::ApiError,
        response::Response,
    },
    utils::datetime::{convert_to_date_time, now, today},
};

use super::util_service::{
    date_time_are_in_order, record_belongs_to_baby, records_is_not_empty, uncover_date,
};

pub async fn post_meal_service(new_meal: InputMealDto, baby_id: i32) -> Result<Response, ApiError> {
    let timestamp = uncover_date(new_meal.date)?;
    let timestamp_to_time = uncover_date(new_meal.to_time)?;
    let meal = InsertableMeal::new(
        baby_id,
        timestamp.unwrap_or(now()),
        new_meal.quantity,
        timestamp_to_time,
    );
    ingest_meal(meal)?;
    Ok(Response::NewRecord)
}

pub async fn patch_meal_service(
    meal: InputMealDto,
    record: i32,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let old_meal = find_meal_by_id(record)?;
    record_belongs_to_baby(old_meal.baby_id(), baby_id)?;
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
    let update_meal = UpdateMeal {
        date: new_date,
        quantity: new_quantity,
        to_time: new_to_time,
    };
    patch_meal_record(record, update_meal)?;
    Ok(Response::UpdateRecord)
}

pub async fn filter_meals_by_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let current = pagination.page();
    let (meals, total_pages) = meals_paginated_from_db(baby_id, from_date, to_date, pagination)?;
    let dreams: Vec<MealDto> = into_meals_dto(meals)?;
    let pager = PageInfo::new(current, total_pages);
    let response = PagedResponse::new(dreams, pager);
    Ok(response)
}

pub async fn filter_meals_by_last_days(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let from_date = today()
        .checked_sub_days(Days::new(last_days.into()))
        .unwrap();
    filter_meals_by_range(baby_id, from_date, today(), pagination).await
}

fn into_meals_dto(meals: Vec<Meal>) -> Result<Vec<MealDto>, ApiError> {
    Ok(records_is_not_empty(meals)?
        .into_iter()
        .map(|dream| dream.into())
        .collect())
}

pub async fn delete_meal_service(record: i32, baby_id: i32) -> Result<Response, ApiError> {
    let meal_to_delete = find_meal_by_id(record)?;
    record_belongs_to_baby(meal_to_delete.baby_id(), baby_id)?;
    delete_meal_from_db(record)?;
    Ok(Response::DeleteRecord)
}

pub async fn get_all_meals_paginated_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let current = pagination.page();
    let (meals, total_pages) = get_all_meals_from_baby(baby_id, pagination)?;
    let meals = into_meals_dto(meals)?;
    let pager = PageInfo::new(current, total_pages);
    let response = PagedResponse::new(meals, pager);
    Ok(response)
}
