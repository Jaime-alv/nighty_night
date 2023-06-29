use crate::{
    data::{
        traits::Mandatory,
        user_dto::{FindUserDto, LoginDto, NewUserDto, UserDto},
    },
    error::error::ApiError,
    mapping::user_mapper::VecUser,
    model::{role_model::Rol, user_model::User},
    repository::user_repository::{
        create_user, load_user_by_id, load_user_by_username, query_users,
    },
    utils::validator::{valid_password, validate_fields},
};

use super::association_service::add_rol_to_user_service;

pub async fn create_user_service(new_user: NewUserDto) -> Result<(UserDto, i32), ApiError> {
    if validate_fields(&new_user.data()) {
        return Err(ApiError::EmptyBody);
    }
    if valid_password(&new_user.password) {
        return Err(ApiError::Generic400Error("Password too short.".into()));
    }
    let user = create_user(new_user).await?;
    assign_rol_as_user(user.id()).await?;
    Ok((UserDto::from(&user), user.id()))
}

async fn assign_rol_as_user(user_id: i32) -> Result<(), ApiError> {
    add_rol_to_user_service(user_id, Rol::User).await
}

pub async fn get_all_users_service() -> Result<Vec<UserDto>, ApiError> {
    let users = query_users().await?;
    Ok(VecUser::new(users).into())
}

pub async fn find_user_service(user: FindUserDto) -> Result<UserDto, ApiError> {
    let user = load_user_by_username(&user.username).await?;
    Ok(user.into())
}

pub async fn login_service(login: LoginDto) -> Result<(UserDto, i32), ApiError> {
    if validate_fields(&login.data()) {
        return Err(ApiError::EmptyBody);
    }
    let current_user = match load_user_by_username(&login.username).await {
        Ok(u) => u,
        Err(_) => return Err(ApiError::IncorrectPassword),
    };
    if !current_user.active() {
        return Err(ApiError::NoActiveUser);
    }
    if current_user.is_password_match(&login.password) {
        return Ok((UserDto::from(&current_user), current_user.id()));
    }
    Err(ApiError::IncorrectPassword)
}

pub async fn find_user_by_id_service(user_id: i32) -> Result<User, ApiError> {
    Ok(load_user_by_id(user_id).await?)
}

pub async fn find_user_by_username_service(username: &String) -> Result<User, ApiError> {
    Ok(load_user_by_username(username).await?)
}
