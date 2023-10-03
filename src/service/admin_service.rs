use crate::{
    data::common_structure::RoleDto,
    repository::admin_repository::{select_stats_from_tables, select_roles, StatsDB},
    response::{error::ApiError, response::RecordResponse},
};

pub async fn get_stats_of_tables_service() -> Result<RecordResponse<StatsDB<'static>>, ApiError> {
    let count = select_stats_from_tables()?;
    let response = RecordResponse::new(count);
    Ok(response)
}

pub async fn get_roles_service() -> Result<RecordResponse<Vec<RoleDto>>, ApiError> {
    let grouped_data = select_roles()?;
    let data = grouped_data
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<RoleDto>>();
    let response = RecordResponse::new(data);
    Ok(response)
}
