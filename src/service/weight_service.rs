use chrono::{Days, NaiveDate};

use crate::{
    data::{common_structure::WeightDto, query_dto::Pagination, weight_dto::InputWeightDto},
    model::weight_model::{InsertableWeight, Weight},
    repository::weight_repository::{
        delete_weight, insert_new_weight, select_all_weights_from_baby, select_weight_by_id,
        select_weights_with_pagination, update_weight,
    },
    response::{
        error::ApiError,
        response::PagedResponse,
        response::{MsgResponse, RecordResponse},
    },
    utils::datetime::{convert_to_date, today},
};

use super::util_service::assert_record_belongs_to_parent;

pub async fn post_weight_service(
    new_measure: InputWeightDto,
    baby_id: i32,
) -> Result<RecordResponse<WeightDto>, ApiError> {
    let date = match new_measure.date {
        Some(day) => convert_to_date(&day)?,
        None => today(),
    };
    let measure = InsertableWeight::new(baby_id, date, new_measure.value.unwrap_or_default());
    let entry: Weight = insert_new_weight(measure)?;
    let response: RecordResponse<WeightDto> = RecordResponse::new_entry(entry.into());
    Ok(response)
}

pub async fn get_weights_all_service(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let current = pagination.page();
    let (measures, total_pages) = select_all_weights_from_baby(baby_id, pagination)?;
    let measures = into_weight_dto(measures)?;
    let response = PagedResponse::new(measures, current, total_pages);
    Ok(response)
}

pub async fn get_weight_range_service(
    baby_id: i32,
    from: NaiveDate,
    to: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let current = pagination.page();
    let (measures, total_pages) = select_weights_with_pagination(baby_id, from, to, pagination)?;
    let measures = into_weight_dto(measures)?;
    let response = PagedResponse::new(measures, current, total_pages);
    Ok(response)
}

pub async fn get_weights_by_last_days(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<WeightDto>>, ApiError> {
    let today = today();
    let from = today.checked_sub_days(Days::new(last_days.into())).unwrap();
    get_weight_range_service(baby_id, from, today, pagination).await
}

pub async fn patch_weight_service(
    measure: InputWeightDto,
    record: i32,
    baby_id: i32,
) -> Result<RecordResponse<WeightDto>, ApiError> {
    let old_record = select_weight_by_id(record)?;
    assert_record_belongs_to_parent(old_record.baby_id(), baby_id)?;
    let weight: Weight = update_weight(old_record.update_weight(measure))?;
    let response: RecordResponse<WeightDto> = RecordResponse::new(weight.into());
    Ok(response)
}

fn into_weight_dto(measures: Vec<Weight>) -> Result<Vec<WeightDto>, ApiError> {
    Ok(measures.into_iter().map(|measure| measure.into()).collect())
}

pub async fn delete_weight_service(record: i32, baby_id: i32) -> Result<MsgResponse, ApiError> {
    let delete_record = select_weight_by_id(record)?;
    assert_record_belongs_to_parent(delete_record.baby_id(), baby_id)?;
    delete_weight(record)?;
    Ok(MsgResponse::DeleteRecord)
}

pub async fn get_weight_id_service(
    weight_id: i32,
    baby_id: i32,
) -> Result<RecordResponse<WeightDto>, ApiError> {
    let weight: Weight = select_weight_by_id(weight_id)?;
    assert_record_belongs_to_parent(weight.baby_id(), baby_id)?;
    let response: RecordResponse<WeightDto> = RecordResponse::new(weight.into());
    Ok(response)
}
