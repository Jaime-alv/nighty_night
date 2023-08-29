use crate::{
    model::role_model::Rol,
    repository::association_repository::{add_baby_to_user, add_rol_to_user, delete_rol_to_user},
    response::{error::ApiError, response::MsgResponse},
};

use super::user_service::find_user_id_from_username;

pub async fn add_rol_to_user_service(username: &str, rol: Rol) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    match add_rol_to_user(user, rol.into()) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn add_baby_to_user_service(
    baby_id: i32,
    username: &str,
) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    match add_baby_to_user(user, baby_id) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn delete_rol_to_user_service(username: &str, rol: Rol) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    match delete_rol_to_user(user, rol.into()) {
        Ok(_) => Ok(MsgResponse::DeleteRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}
