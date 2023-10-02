use crate::{
    model::role_model::Rol,
    repository::association_repository::{delete_rol_to_user, insert_rol_to_user},
    response::{error::ApiError, response::MsgResponse},
};

pub async fn add_rol_to_user_service(user: i32, rol: Rol) -> Result<MsgResponse, ApiError> {
    match insert_rol_to_user(user, rol.into()) {
        Ok(_) => Ok(MsgResponse::UpdateRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn delete_rol_to_user_service(user: i32, rol: Rol) -> Result<MsgResponse, ApiError> {
    match delete_rol_to_user(user, rol.into()) {
        Ok(_) => Ok(MsgResponse::DeleteRecord),
        Err(error) => Err(ApiError::DBError(error)),
    }
}
