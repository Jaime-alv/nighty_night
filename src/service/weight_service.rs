use crate::{
    data::weight_dto::{NewWeightDto, WeightDto},
    error::error::ApiError,
    mapping::weight_mapper::VecWeight,
    model::weight_model::InsertableWeight,
    repository::weight_repository::{get_all_weights_from_baby, ingest_weight},
    utils::{datetime::to_date, response::Response},
};

use super::response_service::ok;

pub async fn post_weight_service(
    new_measure: NewWeightDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let date = to_date(&new_measure.date)?;
    let measure = InsertableWeight::new(baby_id, date, new_measure.value);
    ingest_weight(measure).await?;
    Ok(ok("New measure added"))
}

pub async fn get_weights_service(baby_id: i32) -> Result<Vec<WeightDto>, ApiError> {
    let measures = get_all_weights_from_baby(baby_id).await?;
    Ok(VecWeight::new(measures).into())
}
