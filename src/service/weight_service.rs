use axum::Json;

use crate::{
    data::weight_dto::{NewWeightDto, UpdateWeightDto, WeightDto},
    error::error::ApiError,
    model::weight_model::{InsertableWeight, Weight},
    repository::weight_repository::{
        find_weight_by_id, get_all_weights_from_baby, ingest_weight, patch_weight_record,
    },
    utils::{datetime::convert_to_date, response::Response},
};

pub async fn post_weight_service(
    new_measure: NewWeightDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let date = convert_to_date(&new_measure.date)?;
    let measure = InsertableWeight::new(baby_id, date, new_measure.value);
    ingest_weight(measure).await?;
    Ok(Response::NewRecord)
}

pub async fn get_weights_service(baby_id: i32) -> Result<Json<Vec<WeightDto>>, ApiError> {
    let measures = get_all_weights_from_baby(baby_id).await?;
    Ok(into_json(measures))
}

pub async fn patch_weight_service(
    measure: UpdateWeightDto,
    record: i32,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let old_record = find_weight_by_id(record).await?;
    if old_record.baby_id().ne(&baby_id) {
        return Err(ApiError::Forbidden);
    }
    let new_date = match measure.date {
        Some(value) => convert_to_date(&value)?,
        None => old_record.date(),
    };
    let new_measure = match measure.value {
        Some(value) => value,
        None => old_record.value(),
    };
    let new_weight = Weight::new(old_record.id(), old_record.baby_id(), new_date, new_measure);
    patch_weight_record(new_weight).await?;
    Ok(Response::UpdateRecord)
}

fn into_json(weights: Vec<Weight>) -> Json<Vec<WeightDto>> {
    Json(weights.into_iter().map(|measure| measure.into()).collect())
}
