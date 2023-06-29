use crate::{
    error::error::ApiError,
    model::role_model::Rol,
    repository::association_repository::{add_baby_to_user, add_rol_to_user},
};

pub async fn add_rol_to_user_service<T>(user_id: T, rol: Rol) -> Result<(), ApiError>
where
    T: Into<i32>,
{
    add_rol_to_user(user_id.into(), rol.into()).await?;
    Ok(())
}

pub async fn add_baby_to_user_service<T>(user_id: T, baby_id: T) -> Result<(), ApiError>
where
    T: Into<i32>,
{
    add_baby_to_user(user_id.into(), baby_id.into()).await?;
    Ok(())
}
