use chrono::NaiveDateTime;
use hyper::StatusCode;

use crate::{
    data::dream_dto::{DreamDto, NewDreamDto},
    error::error::ApiError,
    mapping::dream_mapper::from_dream_to_dream_dto_vector,
    model::dream_model::InsertableDream,
    repository::dream_repository::{get_all_dreams_from_baby, ingest_new_dream, update_last_dream},
    utils::{datetime::to_date, response::Response},
};

pub async fn post_dream_service(
    new_dream: NewDreamDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let dream: InsertableDream;
    if new_dream.from_date.is_some() {
        dream = create_new_dream_entry(new_dream, baby_id).await;
        match ingest_new_dream(dream) {
            Ok(_) => return Ok(Response::new(StatusCode::OK, "New dream added.")),
            Err(error) => return Err(ApiError::DBError(error)),
        }
    } else {
        dream = create_new_dream_entry(new_dream, baby_id).await;
        match update_last_dream(dream) {
            Ok(_) => return Ok(Response::new(StatusCode::OK, "Dream update.")),
            Err(error) => return Err(ApiError::DBError(error)),
        }
    }
}

async fn create_new_dream_entry(new_dream: NewDreamDto, baby_id: i32) -> InsertableDream {
    let to_date_binding = uncover_date(new_dream.to_date).await;
    let from_date_binding = uncover_date(new_dream.from_date).await;
    let dream = InsertableDream::new(baby_id, from_date_binding, to_date_binding);
    dream
}

async fn uncover_date(date: Option<String>) -> Option<NaiveDateTime> {
    match date {
        Some(d) => Some(to_date(&d).expect("Wrong date format.")),
        None => None,
    }
}

pub async fn get_all_dreams_from_baby_service(baby_id: i32) -> Result<Vec<DreamDto>, ApiError> {
    match get_all_dreams_from_baby(baby_id) {
        Ok(dreams) => Ok(from_dream_to_dream_dto_vector(dreams).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}
