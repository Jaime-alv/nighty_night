use axum::Json;

use crate::{
    data::{
        admin_dto::AdminBabyDto,
        baby_dto::{BabyDto, InputBabyDto, UpdateBaby},
        query_dto::Pagination,
    },
    error::error::ApiError,
    model::baby_model::InsertableBaby,
    repository::{
        association_repository::delete_baby_association,
        baby_repository::{
            delete_baby_from_db, get_all_babies_with_id, ingest_new_baby_in_db, load_baby_by_id,
            patch_baby_record, query_babies,
        },
    },
    utils::{
        datetime::{convert_to_date, today},
        response::Response,
    },
};

use super::{session_service::load_user_session, util_service::records_is_not_empty};

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
    let baby = ingest_new_baby_in_db(insert_baby, current_user.into())?;
    Ok(Json(baby.into()))
}

pub async fn find_baby_service(baby_id: i32) -> Result<Json<BabyDto>, ApiError> {
    let baby = load_baby_by_id(baby_id)?;
    Ok(Json(baby.into()))
}

pub async fn get_all_babies_service(
    pagination: Pagination,
) -> Result<Json<Vec<AdminBabyDto>>, ApiError> {
    let (babies, _pages) = query_babies(pagination)?;
    Ok(Json(
        records_is_not_empty(babies)?
            .into_iter()
            .map(|baby| baby.into())
            .collect(),
    ))
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

/*
If baby belongs to current user, delete everything from said baby, if not, delete only the
association between user and baby.
*/
pub async fn delete_baby_service(baby_id: i32, user: i32) -> Result<Response, ApiError> {
    let baby = load_baby_by_id(baby_id)?;
    match baby.belongs_to().eq(&user) {
        true => {
            delete_baby_from_db(baby_id)?;
            Ok(Response::DeleteRecord)
        }
        false => {
            delete_baby_association(baby_id, user)?;
            Ok(Response::DeleteRecord)
        }
    }
}

pub async fn load_babies_for_current_user(
    user_id: i64,
    pagination: Pagination,
) -> Result<Json<Vec<BabyDto>>, ApiError> {
    let user = load_user_session(user_id).await?;
    let (babies, _last_page) = get_all_babies_with_id(user.baby_id(), pagination)?;
    Ok(Json(
        records_is_not_empty(babies)?
            .into_iter()
            .map(|baby| baby.into())
            .collect(),
    ))
}
