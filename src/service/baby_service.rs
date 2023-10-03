use crate::{
    data::{
        baby_dto::InputBabyDto,
        common_structure::{AdminBabyDto, BabyDto},
        query_dto::Pagination,
    },
    model::baby_model::InsertableBaby,
    repository::{
        association_repository::{delete_baby_association, insert_baby_to_user},
        baby_repository::{
            delete_baby_from_db, insert_new_baby, select_babies, select_babies_from_user_id,
            select_baby_by_id, update_baby, update_baby_belongs_to,
        },
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    },
    utils::datetime::{convert_to_date, today},
};

pub async fn post_new_baby_service<T>(
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
    let baby: BabyDto = insert_new_baby(insert_baby, user)?.into();
    Ok(RecordResponse::new_entry(baby.into()))
}

pub async fn get_baby_by_id_service(baby_id: i32) -> Result<RecordResponse<BabyDto>, ApiError> {
    let baby = select_baby_by_id(baby_id)?;
    Ok(RecordResponse::new(baby.into()))
}

pub async fn get_all_babies_service(
    pagination: Pagination,
) -> Result<PagedResponse<Vec<AdminBabyDto>>, ApiError> {
    let current = pagination.page();
    let (babies, total_pages) = select_babies(pagination)?;
    let babies: Vec<AdminBabyDto> = babies.into_iter().map(|baby| baby.into()).collect();
    let response = PagedResponse::new(babies, current, total_pages);
    Ok(response)
}

pub async fn patch_baby_service(
    baby_id: i32,
    update: InputBabyDto,
) -> Result<RecordResponse<BabyDto>, ApiError> {
    let baby = select_baby_by_id(baby_id)?;
    let updated_baby = update_baby(baby.update_baby(update))?;
    let response = RecordResponse::new(updated_baby.into());
    Ok(response)
}

/*
If baby belongs to current user, delete everything from said baby, if not, delete only the
association between user and baby.
*/
pub async fn delete_baby_service(baby_id: i32, user: i32) -> Result<MsgResponse, ApiError> {
    let baby = select_baby_by_id(baby_id)?;
    match baby.belongs_to().eq(&user) {
        true => delete_baby_from_db(baby_id)?,
        false => delete_baby_association(baby_id, user)?,
    };
    Ok(MsgResponse::DeleteRecord)
}

pub async fn get_babies_for_user_service(
    user_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<BabyDto>>, ApiError> {
    let current = pagination.page();
    let (babies, total_pages) = select_babies_from_user_id(user_id, pagination)?;
    let babies: Vec<BabyDto> = babies.into_iter().map(|baby| baby.into()).collect();
    let response = PagedResponse::new(babies, current, total_pages);
    Ok(response)
}

/// Change ownership from one user to another.
pub async fn transfer_baby_service(baby_id: i32, user_id: i32) -> Result<MsgResponse, ApiError> {
    insert_baby_to_user(user_id, baby_id)?;
    match update_baby_belongs_to(baby_id, user_id) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(error.into()),
    }
}

/// Share baby record with another user.
pub async fn post_share_baby_with_user_service(
    baby_id: i32,
    user: i32,
) -> Result<MsgResponse, ApiError> {
    match insert_baby_to_user(user, baby_id) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}