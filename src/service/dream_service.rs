use crate::{
    data::dream_dto::{DreamDto, DreamSummary, NewDreamDto},
    error::error::ApiError,
    mapping::dream_mapper::VecDream,
    model::dream_model::InsertableDream,
    repository::dream_repository::{
        find_dreams_by_date, get_all_dreams_from_baby, ingest_new_dream, update_last_dream,
    },
    utils::{
        datetime::{format_date, format_duration, to_date},
        response::Response,
    },
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

pub async fn get_all_dreams_from_baby_service(baby_id: i32) -> Result<Vec<DreamDto>, ApiError> {
    let dreams = get_all_dreams_from_baby(baby_id).await?;
    Ok(VecDream::new(dreams).into())
}

pub async fn filter_dreams_by_date_service(
    baby_id: i32,
    string_date: &str,
) -> Result<Vec<DreamDto>, ApiError> {
    let date = to_date(string_date)?;
    let dreams = find_dreams_by_date(baby_id, date).await?;
    Ok(VecDream::new(dreams).into())
}

pub async fn dream_summary_service(
    baby_id: i32,
    string_date: &str,
) -> Result<DreamSummary, ApiError> {
    let date = to_date(string_date)?;
    let selected_dreams = find_dreams_by_date(baby_id, date).await?;
    let sum_duration = selected_dreams
        .iter()
        .map(|d| d.elapsed())
        .reduce(|acc, e| acc.checked_add(&e).unwrap())
        .unwrap();
    Ok(DreamSummary {
        date: format_date(date),
        summary: format_duration(sum_duration.num_minutes()),
    })
}
