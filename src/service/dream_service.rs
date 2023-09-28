use chrono::{Days, NaiveDate};

use crate::{
    data::{common_structure::DreamDto, dream_dto::InputDreamDto, query_dto::Pagination},
    model::dream_model::{Dream, InsertableDream},
    repository::dream_repository::{
        delete_dream, insert_new_dream, select_all_dreams_from_baby, select_dream_by_id,
        select_dreams_with_pagination, update_dream, update_last_dream,
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    },
    utils::datetime::today,
};

use super::util_service::{assert_record_belongs_to_parent, cast_to_date_from};

pub async fn post_dream_service(
    new_dream: InputDreamDto,
    baby_id: i32,
) -> Result<RecordResponse<DreamDto>, ApiError> {
    let dream: InsertableDream;
    let entry: Dream = if new_dream.from_date.is_some() {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        let entry: Dream = insert_new_dream(dream)?;
        entry
    } else {
        dream = create_new_dream_entry(new_dream, baby_id).await?;
        let entry: Dream = update_last_dream(dream)?;
        entry
    };
    let response: RecordResponse<DreamDto> = RecordResponse::new_entry(entry.into());
    Ok(response)
}

async fn create_new_dream_entry(
    new_dream: InputDreamDto,
    baby_id: i32,
) -> Result<InsertableDream, ApiError> {
    let to_date_binding = cast_to_date_from(new_dream.to_date)?;
    let from_date_binding = cast_to_date_from(new_dream.from_date)?;
    let dream = InsertableDream::new(baby_id, from_date_binding, to_date_binding);
    Ok(dream)
}

pub async fn patch_dream_service(
    dream: InputDreamDto,
    record: i32,
    baby_id: i32,
) -> Result<RecordResponse<DreamDto>, ApiError> {
    let dream_record = select_dream_by_id(record)?;
    assert_record_belongs_to_parent(dream_record.baby_id(), baby_id)?;

    let dream: Dream = update_dream(dream_record.update_dream(dream))?;
    let response: RecordResponse<DreamDto> = RecordResponse::new(dream.into());
    Ok(response)
}

pub async fn get_dreams_by_range_date_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamDto>>, ApiError> {
    let current = pagination.page();
    let (dreams, total_pages) =
        select_dreams_with_pagination(baby_id, pagination, from_date, to_date)?;
    let dreams: Vec<DreamDto> = into_dreams_dto(dreams)?;
    let response = PagedResponse::new(dreams, current, total_pages);
    Ok(response)
}

pub async fn get_dreams_by_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamDto>>, ApiError> {
    let today = today();
    let from_date = today.checked_sub_days(Days::new(last_days.into())).unwrap();
    get_dreams_by_range_date_service(baby_id, from_date, today, pagination).await
}

fn into_dreams_dto(dreams: Vec<Dream>) -> Result<Vec<DreamDto>, ApiError> {
    Ok(dreams.into_iter().map(|dream| dream.into()).collect())
}

pub async fn delete_dream_service(record: i32, baby_id: i32) -> Result<MsgResponse, ApiError> {
    let old_dream = select_dream_by_id(record)?;
    assert_record_belongs_to_parent(old_dream.baby_id(), baby_id)?;
    delete_dream(record)?;
    Ok(MsgResponse::DeleteRecord)
}

pub async fn get_dreams_all_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamDto>>, ApiError> {
    let current = pagination.page();
    let (dreams, total_pages) = select_all_dreams_from_baby(baby_id, pagination)?;
    let dreams: Vec<DreamDto> = into_dreams_dto(dreams)?;
    let response = PagedResponse::new(dreams, current, total_pages);
    Ok(response)
}

pub async fn get_dream_id_service(
    dream_id: i32,
    baby_id: i32,
) -> Result<RecordResponse<DreamDto>, ApiError> {
    let dream: Dream = select_dream_by_id(dream_id)?;
    assert_record_belongs_to_parent(dream.baby_id(), baby_id)?;
    let response: RecordResponse<DreamDto> = RecordResponse::new(dream.into());
    Ok(response)
}
