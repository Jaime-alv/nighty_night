use crate::{
    error::error::ApiError, model::role_model::Rol,
    repository::association_repository::{add_rol_to_user, add_baby_to_user},
};

pub async fn add_rol_to_user_service<T>(user_id: T, rol: Rol) -> Result<(), ApiError>
where
    T: Into<i32>,
{
    match add_rol_to_user(user_id.into(), rol.into()) {
        Ok(_) => Ok(()),
        Err(error) => Err(ApiError::DBError(error)),
    }
}

pub async fn add_baby_to_user_service<T>(user_id: T, baby_id: T) -> Result<(), ApiError>
where
    T: Into<i32>,
{
    match add_baby_to_user(user_id.into(), baby_id.into()) {
        Ok(_) => Ok(()),
        Err(error) => Err(ApiError::DBError(error)),
    }
}
