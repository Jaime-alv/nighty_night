use chrono::Duration;

use crate::{
    data::dream_dto::{DreamDto, DreamSummary, NewDreamDto},
    error::error::ApiError,
    mapping::dream_mapper::from_dream_to_dream_dto_vector,
    model::dream_model::InsertableDream,
    repository::dream_repository::{
        find_dreams_by_date, get_all_dreams_from_baby, ingest_new_dream, update_last_dream,
    },
    utils::{
        datetime::{format_date, format_duration},
        response::Response,
    },
};

use super::{
    date_service::{parse_date, uncover_date},
    response_service::ok,
};

pub async fn post_dream_service(
    new_dream: NewDreamDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let dream: InsertableDream;
    if new_dream.from_date.is_some() {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        match ingest_new_dream(dream).await {
            Ok(_) => return Ok(ok("New dream added.")),
            Err(error) => return Err(ApiError::DBError(error)),
        }
    } else {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        match update_last_dream(dream).await {
            Ok(_) => return Ok(ok("Dream update.")),
            Err(error) => return Err(ApiError::DBError(error)),
        }
    }
}

async fn create_new_dream_entry(
    new_dream: NewDreamDto,
    baby_id: i32,
) -> Result<InsertableDream, ApiError> {
    let to_date_binding = uncover_date(new_dream.to_date)?;
    let from_date_binding = uncover_date(new_dream.from_date)?;
    let dream = InsertableDream::new(baby_id, from_date_binding, to_date_binding);
    Ok(dream)
}

pub async fn get_all_dreams_from_baby_service(baby_id: i32) -> Result<Vec<DreamDto>, ApiError> {
    match get_all_dreams_from_baby(baby_id).await {
        Ok(dreams) => Ok(from_dream_to_dream_dto_vector(dreams).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn filter_dreams_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Vec<DreamDto>, ApiError> {
    let date = parse_date(string_date)?;
    match find_dreams_by_date(baby_id, date).await {
        Ok(dreams) => Ok(from_dream_to_dream_dto_vector(dreams).await),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn dream_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<DreamSummary, ApiError> {
    let date = parse_date(string_date)?;
    let selected_dreams = match find_dreams_by_date(baby_id, date).await {
        Ok(dreams) => dreams
            .iter()
            .map(|d| d.elapsed())
            .collect::<Vec<Duration>>(),
        Err(error) => return Err(ApiError::DBError(error)),
    };
    let duration = selected_dreams
        .into_iter()
        .reduce(|acc, e| acc.checked_add(&e).unwrap())
        .unwrap();
    Ok(DreamSummary {
        date: format_date(date),
        summary: format_duration(duration.num_minutes()),
    })
}
