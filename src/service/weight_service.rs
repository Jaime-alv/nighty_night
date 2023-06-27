use crate::{
    data::weight_dto::{NewWeightDto, WeightDto},
    error::error::ApiError,
    model::weight_model::InsertableWeight,
    utils::{datetime::to_date, response::Response}, repository::weight_repository::{ingest_weight, get_all_weights_from_baby}, mapping::weight_mapper::from_weight_to_weight_dto_vector,
};

use super::response_service::ok;

pub async fn post_weight_service(
    new_measure: NewWeightDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let measure = InsertableWeight::new(baby_id, to_date(&new_measure.date), new_measure.value);
    match ingest_weight(measure).await {
        Ok(_) => Ok(ok("New measure added").await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn get_weights_service(baby_id: i32) -> Result<Vec<WeightDto>, ApiError> {
    match get_all_weights_from_baby(baby_id).await {
        Ok(measures) => Ok(from_weight_to_weight_dto_vector(measures).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}