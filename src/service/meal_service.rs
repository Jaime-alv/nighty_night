use chrono::{Days, NaiveDate};

use crate::{
    data::{common_structure::MealDto, meal_dto::InputMealDto, query_dto::Pagination},
    model::meals_model::{InsertableMeal, Meal},
    repository::meal_repository::{
        delete_meal, insert_new_meal, select_all_meals_from_baby, select_meal_by_id,
        select_meals_with_pagination, update_meal,
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    },
    utils::datetime::{now, today},
};

use super::util_service::{assert_record_belongs_to_parent, cast_to_date_from};

pub async fn post_meal_service(
    new_meal: InputMealDto,
    baby_id: i32,
) -> Result<RecordResponse<MealDto>, ApiError> {
    let timestamp = cast_to_date_from(new_meal.date)?;
    let timestamp_to_time = cast_to_date_from(new_meal.to_time)?;
    let meal = InsertableMeal::new(
        baby_id,
        timestamp.unwrap_or(now()),
        new_meal.quantity,
        timestamp_to_time,
    );
    let insert_data: Meal = insert_new_meal(meal)?;
    let response: RecordResponse<MealDto> = RecordResponse::new_entry(insert_data.into());
    Ok(response)
}

pub async fn patch_meal_service(
    meal: InputMealDto,
    record: i32,
    baby_id: i32,
) -> Result<MsgResponse, ApiError> {
    let meal_record = select_meal_by_id(record)?;
    assert_record_belongs_to_parent(meal_record.baby_id(), baby_id)?;
    update_meal(meal_record.update_meal(meal))?;
    Ok(MsgResponse::UpdateRecord)
}

pub async fn get_meals_by_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<MealDto>>, ApiError> {
    let current = pagination.page();
    let (meals, total_pages) =
        select_meals_with_pagination(baby_id, from_date, to_date, pagination)?;
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
    assert_record_belongs_to_parent(meal_to_delete.baby_id(), baby_id)?;
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

pub async fn get_meal_id_service(
    meal_id: i32,
    baby_id: i32,
) -> Result<RecordResponse<MealDto>, ApiError> {
    let meal = select_meal_by_id(meal_id)?;
    assert_record_belongs_to_parent(meal.baby_id(), baby_id)?;
    let response: RecordResponse<MealDto> = RecordResponse::new(meal.into());
    Ok(response)
}
