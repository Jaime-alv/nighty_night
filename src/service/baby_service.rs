use axum::Json;

use crate::{
    data::baby_dto::{BabyDto, InputBabyDto, UpdateBaby},
    error::error::ApiError,
    model::baby_model::{Baby, InsertableBaby},
    repository::baby_repository::{
        delete_baby_from_db, ingest_new_baby_in_db, load_baby_by_id, patch_baby_record,
        query_babies,
    },
    utils::{
        datetime::{convert_to_date, today},
        response::Response,
    },
};

use super::association_service::add_baby_to_user_service;

pub async fn ingest_new_baby<T>(
    new_baby: InputBabyDto,
    current_user: T,
) -> Result<Json<BabyDto>, ApiError>
where
    T: Into<i32>,
{
    if new_baby.name.is_none() {
        return Err(ApiError::EmptyBody);
    }
    let birthdate = match new_baby.birthdate {
        Some(day) => convert_to_date(&day)?,
        None => today(),
    };
    let insert_baby = InsertableBaby::new(new_baby.name.unwrap(), birthdate);
    let baby = ingest_new_baby_in_db(insert_baby)?;
    add_baby_to_user_service(current_user.into(), baby.id().into()).await?;
    Ok(Json(baby.into()))
}

pub async fn find_baby_service(baby_id: i32) -> Result<Json<BabyDto>, ApiError> {
    let baby = load_baby_by_id(baby_id)?;
    Ok(Json(baby.into()))
}

pub async fn get_all_babies_service() -> Result<Json<Vec<BabyDto>>, ApiError> {
    let babies = query_babies()?;
    Ok(into_json(babies))
}

fn into_json(babies: Vec<Baby>) -> Json<Vec<BabyDto>> {
    Json(babies.into_iter().map(|baby| baby.into()).collect())
}

pub async fn patch_baby_service(baby_id: i32, update: InputBabyDto) -> Result<Response, ApiError> {
    let old_record = load_baby_by_id(baby_id)?;
    let new_name = match update.name {
        Some(value) => value,
        None => old_record.name(),
    };
    let new_birthdate = match update.birthdate {
        Some(day) => convert_to_date(&day)?,
        None => old_record.birthdate(),
    };
    let update_baby = UpdateBaby {
        name: new_name,
        birthdate: new_birthdate,
    };
    patch_baby_record(baby_id, update_baby)?;
    Ok(Response::UpdateRecord)
}

pub async fn delete_baby_service(baby_id: i32) -> Result<Response, ApiError> {
    delete_baby_from_db(baby_id)?;
    Ok(Response::DeleteRecord)
}
