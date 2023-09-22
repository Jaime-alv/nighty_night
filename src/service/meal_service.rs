use chrono::{Days, NaiveDate};

use crate::{
    data::{
        common_structure::MealDto,
        meal_dto::{InputMealDto, UpdateMeal},
        query_dto::Pagination,
    },
    model::meals_model::{InsertableMeal, Meal},
    repository::meal_repository::{
        delete_meal, select_meal_by_id, select_all_meals_from_baby, insert_new_meal,
        select_meals_with_pagination, update_meal,
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse},
    },
    utils::datetime::{convert_to_date_time, now, today},
};

use super::util_service::{are_date_time_in_order, does_record_belongs_to_baby, cast_to_date_from};

pub async fn post_meal_service(
    new_meal: InputMealDto,
    baby_id: i32,
) -> Result<MsgResponse, ApiError> {
    let timestamp = cast_to_date_from(new_meal.date)?;
    let timestamp_to_time = cast_to_date_from(new_meal.to_time)?;
    let meal = InsertableMeal::new(
        baby_id,
        timestamp.unwrap_or(now()),
        new_meal.quantity,
        timestamp_to_time,
    );
    insert_new_meal(meal)?;
    Ok(MsgResponse::NewRecord)
}

pub async fn patch_meal_service(
    meal: InputMealDto,
    record: i32,
    baby_id: i32,
) -> Result<MsgResponse, ApiError> {
    let old_meal = select_meal_by_id(record)?;
    does_record_belongs_to_baby(old_meal.baby_id(), baby_id)?;
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
            are_date_time_in_order(new_date.clone(), date_time)?;
            Some(date_time)
        }
        None => old_meal.to_time(),
    };
    let update_meal_record = UpdateMeal {
        date: new_date,
        quantity: new_quantity,
        to_time: new_to_time,
    };
    update_meal(record, update_meal_record)?;
    Ok(MsgResponse::UpdateRecord)
}

pub async fn get_meals_by_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let current = pagination.page();
    let (meals, total_pages) = select_meals_with_pagination(baby_id, from_date, to_date, pagination)?;
    let dreams: Vec<MealDto> = into_meals_dto(meals)?;
    let response = PagedResponse::new(dreams, current, total_pages);
    Ok(response)
}

pub async fn get_meals_by_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let from_date = today()
        .checked_sub_days(Days::new(last_days.into()))
        .unwrap();
    get_meals_by_range_service(baby_id, from_date, today(), pagination).await
}

fn into_meals_dto(meals: Vec<Meal>) -> Result<Vec<MealDto>, ApiError> {
    Ok(meals.into_iter().map(|dream| dream.into()).collect())
}

pub async fn delete_meal_service(record: i32, baby_id: i32) -> Result<MsgResponse, ApiError> {
    let meal_to_delete = select_meal_by_id(record)?;
    does_record_belongs_to_baby(meal_to_delete.baby_id(), baby_id)?;
    delete_meal(record)?;
    Ok(MsgResponse::DeleteRecord)
}

pub async fn get_meals_all_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let current = pagination.page();
    let (meals, total_pages) = select_all_meals_from_baby(baby_id, pagination)?;
    let meals = into_meals_dto(meals)?;
    let response = PagedResponse::new(meals, current, total_pages);
    Ok(response)
}
