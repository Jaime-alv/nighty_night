use crate::{
    model::role_model::Rol,
    repository::association_repository::{insert_baby_to_user, insert_rol_to_user, delete_rol_to_user},
    response::{error::ApiError, response::MsgResponse},
};

use super::user_service::find_user_id_from_username;

pub async fn add_rol_to_user_service(username: &str, rol: Rol) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    match insert_rol_to_user(user, rol.into()) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn post_share_baby_with_user_service(
    baby_id: i32,
    username: &str,
) -> Result<MsgResponse, ApiError> {
    let user = find_user_id_from_username(username).await?;
    match insert_baby_to_user(user, baby_id) {
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
