use axum::Json;
use chrono::{Days, NaiveDate};

use crate::{
    data::{
        query_dto::Pagination,
        weight_dto::{InputWeightDto, UpdateWeight, WeightDto},
    },
    error::error::ApiError,
    model::weight_model::{InsertableWeight, Weight},
    repository::weight_repository::{
        delete_weight_from_db, find_weight_by_id, get_all_weights_from_baby, ingest_weight,
        patch_weight_record, weights_paginated_from_db,
    },
    utils::{
        datetime::{convert_to_date, today},
        response::Response,
    },
};

use super::util_service::{record_belongs_to_baby, records_is_not_empty};

pub async fn post_weight_service(
    new_measure: InputWeightDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let date = match new_measure.date {
        Some(day) => convert_to_date(&day)?,
        None => today(),
    };
    let measure = InsertableWeight::new(baby_id, date, new_measure.value.unwrap_or_default());
    ingest_weight(measure)?;
    Ok(Response::NewRecord)
}

pub async fn get_weights_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<Json<Vec<WeightDto>>, ApiError> {
    let (measures, _total_pages) = get_all_weights_from_baby(baby_id, pagination)?;
    Ok(into_json(records_is_not_empty(measures)?))
}

pub async fn get_weight_range_service(
    baby_id: i32,
    from: NaiveDate,
    to: NaiveDate,
    pagination: Pagination,
) -> Result<Json<Vec<WeightDto>>, ApiError> {
    let (measures, _total_pages) = weights_paginated_from_db(baby_id, from, to, pagination)?;
    Ok(into_json(records_is_not_empty(measures)?))
}

pub async fn filter_weights_by_last_days(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<Json<Vec<WeightDto>>, ApiError> {
    let today = today();
    let from = today.checked_sub_days(Days::new(last_days.into())).unwrap();
    get_weight_range_service(baby_id, from, today, pagination).await
}

pub async fn patch_weight_service(
    measure: InputWeightDto,
    record: i32,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let old_record = find_weight_by_id(record)?;
    record_belongs_to_baby(old_record.baby_id(), baby_id)?;
    let new_date = match measure.date {
        Some(value) => convert_to_date(&value)?,
        None => old_record.date(),
    };
    let new_measure = match measure.value {
        Some(value) => value,
        None => old_record.value(),
    };
    let new_weight = UpdateWeight {
        date: new_date,
        value: new_measure,
    };
    patch_weight_record(record, new_weight)?;
    Ok(Response::UpdateRecord)
}

fn into_json(weights: Vec<Weight>) -> Json<Vec<WeightDto>> {
    Json(weights.into_iter().map(|measure| measure.into()).collect())
}

pub async fn delete_weight_service(record: i32, baby_id: i32) -> Result<Response, ApiError> {
    let delete_record = find_weight_by_id(record)?;
    record_belongs_to_baby(delete_record.baby_id(), baby_id)?;
    delete_weight_from_db(record)?;
    Ok(Response::DeleteRecord)
}
