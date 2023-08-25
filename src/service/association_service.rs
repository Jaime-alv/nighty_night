use crate::{
    model::role_model::Rol,
    repository::association_repository::{add_baby_to_user, add_rol_to_user},
    response::{error::ApiError, response::Response},
};

use super::user_service::find_user_by_username_service;

pub async fn add_rol_to_user_service<T>(user_id: T, rol: Rol) -> Result<(), ApiError>
where
    T: Into<i32>,
{
    add_rol_to_user(user_id.into(), rol.into())?;
    Ok(())
}

pub async fn add_baby_to_user_service(baby_id: i32, username: &str) -> Result<Response, ApiError> {
    let user = find_user_by_username_service(username).await?;
    match add_baby_to_user(user.id(), baby_id) {
        Ok(_) => Ok(Response::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}
