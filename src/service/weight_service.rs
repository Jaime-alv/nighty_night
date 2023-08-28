use chrono::{Days, NaiveDate};

use crate::{
    data::{
        query_dto::Pagination,
        weight_dto::{InputWeightDto, UpdateWeight, WeightDto},
    },
    model::weight_model::{InsertableWeight, Weight},
    repository::weight_repository::{
        delete_weight_from_db, find_weight_by_id, get_all_weights_from_baby, ingest_weight,
        patch_weight_record, weights_paginated_from_db,
    },
    response::{error::ApiError, response::MsgResponse, response::PagedResponse},
    utils::datetime::{convert_to_date, today},
};

use super::util_service::{record_belongs_to_baby, records_is_not_empty};

pub async fn post_weight_service(
    new_measure: InputWeightDto,
    baby_id: i32,
) -> Result<MsgResponse, ApiError> {
    let date = match new_measure.date {
        Some(day) => convert_to_date(&day)?,
        None => today(),
    };
    let measure = InsertableWeight::new(baby_id, date, new_measure.value.unwrap_or_default());
    ingest_weight(measure)?;
    Ok(MsgResponse::NewRecord)
}

pub async fn get_weights_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let current = pagination.page();
    let (measures, total_pages) = get_all_weights_from_baby(baby_id, pagination)?;
    let measures = into_weight_dto(measures)?;
    let response = PagedResponse::new(measures, current, total_pages);
    Ok(response)
}

pub async fn get_weight_range_service(
    baby_id: i32,
    from: NaiveDate,
    to: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let current = pagination.page();
    let (measures, total_pages) = weights_paginated_from_db(baby_id, from, to, pagination)?;
    let measures = into_weight_dto(measures)?;
    let response = PagedResponse::new(measures, current, total_pages);
    Ok(response)
}

pub async fn filter_weights_by_last_days(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let today = today();
    let from = today.checked_sub_days(Days::new(last_days.into())).unwrap();
    get_weight_range_service(baby_id, from, today, pagination).await
}

pub async fn patch_weight_service(
    measure: InputWeightDto,
    record: i32,
    baby_id: i32,
) -> Result<MsgResponse, ApiError> {
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
    Ok(MsgResponse::UpdateRecord)
}

fn into_weight_dto(measures: Vec<Weight>) -> Result<Vec<WeightDto>, ApiError> {
    Ok(records_is_not_empty(measures)?
        .into_iter()
        .map(|measure| measure.into())
        .collect())
}

pub async fn delete_weight_service(record: i32, baby_id: i32) -> Result<MsgResponse, ApiError> {
    let delete_record = find_weight_by_id(record)?;
    record_belongs_to_baby(delete_record.baby_id(), baby_id)?;
    delete_weight_from_db(record)?;
    Ok(MsgResponse::DeleteRecord)
}
