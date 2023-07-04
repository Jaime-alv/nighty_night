use axum::Json;

use crate::{
    data::baby_dto::{BabyDto, NewBabyDto},
    error::error::ApiError,
    model::baby_model::Baby,
    repository::baby_repository::{ingest_new_baby_in_db, load_baby_by_id, query_babies},
    utils::datetime::to_date,
};

use super::association_service::add_baby_to_user_service;

pub async fn ingest_new_baby<T>(
    new_baby: NewBabyDto,
    current_user: T,
) -> Result<Json<BabyDto>, ApiError>
where
    T: Into<i32>,
{
    to_date(&new_baby.birthdate)?;
    let baby = ingest_new_baby_in_db(new_baby).await?;
    add_baby_to_user_service(current_user.into(), baby.id().into()).await?;
    Ok(Json(baby.into()))
}

pub async fn find_baby_service(baby_id: i32) -> Result<Json<BabyDto>, ApiError> {
    let baby = load_baby_by_id(baby_id).await?;
    Ok(Json(baby.into()))
}

pub async fn get_all_babies_service() -> Result<Json<Vec<BabyDto>>, ApiError> {
    let babies = query_babies().await?;
    Ok(into_json(babies))
}

fn into_json(babies: Vec<Baby>) -> Json<Vec<BabyDto>> {
    Json(babies.into_iter().map(|baby| baby.into()).collect())
}
