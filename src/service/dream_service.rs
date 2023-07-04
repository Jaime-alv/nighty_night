use axum::Json;

use crate::{
    data::dream_dto::{DreamDto, DreamSummaryDto, NewDreamDto},
    error::error::ApiError,
    model::{
        dream_model::{Dream, InsertableDream},
        summary_model::DreamSummary,
    },
    repository::dream_repository::{
        find_dreams_by_date, get_all_dreams_from_baby, ingest_new_dream, update_last_dream,
    },
    utils::{datetime::to_date, response::Response},
};

use super::util_service::{ok, uncover_date};

pub async fn post_dream_service(
    new_dream: NewDreamDto,
    baby_id: i32,
) -> Result<Response, ApiError> {
    let dream: InsertableDream;
    if new_dream.from_date.is_some() {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        ingest_new_dream(dream).await?;
        Ok(ok("New dream added."))
    } else {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        update_last_dream(dream).await?;
        Ok(ok("Dream update."))
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

pub async fn get_all_dreams_from_baby_service(
    baby_id: i32,
) -> Result<Json<Vec<DreamDto>>, ApiError> {
    let dreams = get_all_dreams_from_baby(baby_id).await?;
    Ok(into_json(dreams))
}

pub async fn filter_dreams_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<Vec<DreamDto>>, ApiError> {
    let date = to_date(string_date)?;
    let dreams = find_dreams_by_date(baby_id, date).await?;
    Ok(into_json(dreams))
}

pub async fn dream_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Json<DreamSummaryDto>, ApiError> {
    let date = to_date(string_date)?;
    let dreams = find_dreams_by_date(baby_id, date).await?;
    let summary = DreamSummary::new(date, dreams);
    Ok(Json(summary.into()))
}

fn into_json(dreams: Vec<Dream>) -> Json<Vec<DreamDto>> {
    Json(dreams.into_iter().map(|dream| dream.into()).collect())
}
