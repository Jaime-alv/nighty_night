use axum::Json;

use crate::{
    data::weight_dto::{NewWeightDto, WeightDto},
    error::error::ApiError,
    model::weight_model::{InsertableWeight, Weight},
    repository::weight_repository::{get_all_weights_from_baby, ingest_weight},
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

fn into_json(weights: Vec<Weight>) -> Json<Vec<WeightDto>> {
    Json(weights.into_iter().map(|measure| measure.into()).collect())
}
