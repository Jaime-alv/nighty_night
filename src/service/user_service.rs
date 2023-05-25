use tracing::error;

use crate::{
    data::{
        traits::Mandatory,
        user_dto::{FindUserDto, LoginDto, NewUserDto, UserDto},
    },
    error::error::ApiError,
    mapping::user_mapper::users_to_users_dto,
    model::user_model::User,
    repository::user_repository::{create_user, exists, load_user, load_user_by_id, query_users},
};

use super::validation::validator::{valid_password, validate_fields};

pub async fn create_user_service(new_user: NewUserDto) -> Result<(UserDto, i32), ApiError> {
    if validate_fields(&new_user.data()) {
        return Err(ApiError::EmptyBody);
    }
    if exists(&new_user.username) {
        return Err(ApiError::DuplicateUser);
    }
    if valid_password(&new_user.password) {
        return Err(ApiError::Generic400Error("Password too short.".into()));
    }
    match create_user(new_user) {
        Ok(user) => Ok((UserDto::from(&user), user.id())),
        Err(msg) => {
            error!("{msg}");
            Err(ApiError::DBError(msg))
        }
    }
}

pub async fn get_all_users_service() -> Result<Vec<UserDto>, ApiError> {
    match query_users() {
        Ok(users) => Ok(users_to_users_dto(users)),
        Err(msg) => {
            error!("{msg}");
            Err(ApiError::DBError(msg))
        }
    }
}

pub async fn find_user_service(user: FindUserDto) -> Result<UserDto, ApiError> {
    match load_user(&user.username) {
        Ok(u) => return Ok(UserDto::from(u)),
        Err(_) => return Err(ApiError::NoUser),
    }
}

pub async fn login_service(login: LoginDto) -> Result<(UserDto, i32), ApiError> {
    if validate_fields(&login.data()) {
        return Err(ApiError::EmptyBody);
    }
    let current_user = match load_user(&login.username) {
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
    match load_user_by_id(user_id) {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiError::NoUser),
    }
}

pub async fn find_user_by_username_service(username: &String) -> Result<User, ApiError> {
    match load_user(username) {
        Ok(u) => return Ok(u),
        Err(_) => return Err(ApiError::NoUser),
    }
}
