use crate::{
    data::role_dto::RoleDto,
    repository::admin_repository::{count_records, select_roles_and_group_by_count, StatsDB},
    response::{error::ApiError, response::RecordResponse},
};

pub async fn show_stats_service() -> Result<RecordResponse<StatsDB<'static>>, ApiError> {
    let count = count_records()?;
    let response = RecordResponse::new(count);
    Ok(response)
}

pub async fn display_roles_service() -> Result<RecordResponse<Vec<RoleDto>>, ApiError> {
    let grouped_data = select_roles_and_group_by_count()?;
    let data = grouped_data
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<RoleDto>>();
    let response = RecordResponse::new(data);
    Ok(response)
}