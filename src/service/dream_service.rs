use axum::Json;
use chrono::NaiveDate;

use crate::{
    data::dream_dto::{DreamDto, InputDreamDto, UpdateDream},
    error::error::ApiError,
    model::dream_model::{Dream, InsertableDream},
    repository::dream_repository::{
        find_dream_by_id, find_dreams_by_date, get_all_dreams_from_baby, ingest_new_dream,
        patch_dream_record, update_last_dream, delete_dream_from_db,
    },
    utils::{datetime::convert_to_date_time, response::Response},
};

use super::util_service::{date_time_are_in_order, uncover_date, record_belongs_to_baby};

pub async fn post_dream_service(
    new_dream: InputDreamDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let dream: InsertableDream;
    if new_dream.from_date.is_some() {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        ingest_new_dream(dream).await?;
        Ok(Response::NewRecord)
    } else {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        update_last_dream(dream).await?;
        Ok(Response::UpdateRecord)
    }
}

async fn create_new_dream_entry(
    new_dream: InputDreamDto,
    baby_id: i32,
) -> Result<InsertableDream, ApiError> {
    let to_date_binding = uncover_date(new_dream.to_date)?;
    let from_date_binding = uncover_date(new_dream.from_date)?;
    let dream = InsertableDream::new(baby_id, from_date_binding, to_date_binding);
    Ok(dream)
}

pub async fn patch_dream_service(
    dream: InputDreamDto,
    record: i32,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let old_dream = find_dream_by_id(record).await?;
    record_belongs_to_baby(old_dream.baby_id(), baby_id)?;
    let new_from_date = match dream.from_date {
        Some(value) => convert_to_date_time(&value)?,
        None => old_dream.from_date(),
    };
    let new_to_date = match dream.to_date {
        Some(value) => {
            let date_time = convert_to_date_time(&value)?;
            date_time_are_in_order(new_from_date, date_time)?;
            Some(date_time)
        }
        None => old_dream.to_date(),
    };
    let update_dream = UpdateDream {
        from_date: new_from_date,
        to_date: new_to_date,
    };
    patch_dream_record(record, update_dream).await?;
    Ok(Response::UpdateRecord)
}

pub async fn get_all_dreams_from_baby_service(
    baby_id: i32,
) -> Result<Json<Vec<DreamDto>>, ApiError> {
    let dreams = get_all_dreams_from_baby(baby_id).await?;
    Ok(into_json(dreams))
}

pub async fn filter_dreams_by_date_service(
    baby_id: i32,
    date: NaiveDate,
) -> Result<Json<Vec<DreamDto>>, ApiError> {
    let dreams = find_dreams_by_date(baby_id, date).await?;
    Ok(into_json(dreams))
}

fn into_json(dreams: Vec<Dream>) -> Json<Vec<DreamDto>> {
    Json(dreams.into_iter().map(|dream| dream.into()).collect())
}


pub async fn delete_dream_service(record: i32, baby_id: i32) -> Result<Response, ApiError> {
    let old_dream = find_dream_by_id(record).await?;
    record_belongs_to_baby(old_dream.baby_id(), baby_id)?;
    delete_dream_from_db(record).await;
    Ok(Response::DeleteRecord)
}