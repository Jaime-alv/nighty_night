use crate::{
    data::{
        baby_dto::{InputBabyDto, UpdateBaby},
        common_structure::{AdminBabyDto, BabyDto},
        query_dto::Pagination,
    },
    model::baby_model::InsertableBaby,
    repository::{
        association_repository::{add_baby_to_user, delete_baby_association},
        baby_repository::{
            delete_baby_from_db, get_all_babies_by_unique_id, ingest_new_baby_in_db,
            load_baby_by_id, patch_baby_record, query_babies, transfer_baby_records,
        },
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    },
    utils::datetime::{convert_to_date, today},
};

use super::{
    session_service::load_user_session, user_service::find_user_id_from_username,
    util_service::records_is_not_empty,
};

pub async fn ingest_new_baby<T>(
    new_baby: InputBabyDto,
    current_user: T,
) -> Result<RecordResponse<BabyDto>, ApiError>
where
    T: Into<i32>,
{
    let user: i32 = current_user.into();
    if new_baby.name.is_none() {
        return Err(ApiError::EmptyBody);
    }
    let birthdate = match new_baby.birthdate {
        Some(day) => convert_to_date(&day)?,
        None => today(),
    };
    let insert_baby = InsertableBaby::new(new_baby.name.unwrap(), birthdate, user);
    let baby: BabyDto = ingest_new_baby_in_db(insert_baby, user)?.into();
    Ok(RecordResponse::new(baby.into()))
}

pub async fn find_baby_service(baby_id: i32) -> Result<RecordResponse<BabyDto>, ApiError> {
    let baby = load_baby_by_id(baby_id)?;
    Ok(RecordResponse::new(baby.into()))
}

pub async fn get_all_babies_service(
    pagination: Pagination,
) -> Result<PagedResponse<Vec<AdminBabyDto>>, ApiError> {
    let current = pagination.page();
    let (babies, total_pages) = query_babies(pagination)?;
    let babies: Vec<AdminBabyDto> = records_is_not_empty(babies)?
        .into_iter()
        .map(|baby| baby.into())
        .collect();
    let response = PagedResponse::new(babies, current, total_pages);
    Ok(response)
}

pub async fn patch_baby_service(
    baby_id: i32,
    update: InputBabyDto,
) -> Result<MsgResponse, ApiError> {
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
    Ok(MsgResponse::UpdateRecord)
}

/*
If baby belongs to current user, delete everything from said baby, if not, delete only the
association between user and baby.
*/
pub async fn delete_baby_service(baby_id: i32, user: i32) -> Result<MsgResponse, ApiError> {
    let baby = load_baby_by_id(baby_id)?;
    match baby.belongs_to().eq(&user) {
        true => delete_baby_from_db(baby_id)?,
        false => delete_baby_association(baby_id, user)?,
    };
    Ok(MsgResponse::DeleteRecord)
}

pub async fn load_babies_for_current_user(
    user_id: i64,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<BabyDto>>, ApiError> {
    let current = pagination.page();
    let user = load_user_session(user_id).await?;
    let (babies, total_pages) = get_all_babies_by_unique_id(user.baby_unique_id(), pagination)?;
    let babies: Vec<BabyDto> = records_is_not_empty(babies)?
        .into_iter()
        .map(|baby| baby.into())
        .collect();
    let response = PagedResponse::new(babies, current, total_pages);
    Ok(response)
}

pub async fn transfer_baby_service(baby_id: i32, username: &str) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    add_baby_to_user(user, baby_id)?;
    match transfer_baby_records(baby_id, user) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(error.into()),
    }
}
