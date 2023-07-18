use std::time::SystemTime;

use axum::Json;

use crate::{
    data::{
        traits::Mandatory,
        user_dto::{FindUserDto, LoginDto, NewUserDto, UpdateUserDto, UserDto, UpdateUser},
    },
    error::error::ApiError,
    model::{role_model::Rol, user_model::User},
    repository::user_repository::{
        create_user, load_user_by_id, load_user_by_username, patch_user_record, query_users,
    },
    utils::{
        datetime::now,
        response::Response,
        validator::{valid_password, validate_fields},
    },
};

use super::{
    association_service::add_rol_to_user_service,
    session_service::{create_current_user, save_user_session},
};

pub async fn create_user_service(new_user: NewUserDto) -> Result<(Response, i32), ApiError> {
    if validate_fields(&new_user.data()) {
        return Err(ApiError::EmptyBody);
    }
    if valid_password(&new_user.password) {
        return Err(ApiError::Generic400Error("Password too short.".into()));
    }
    let user = create_user(new_user).await?;
    assign_rol_as_user(user.id()).await?;
    let username = user.username();
    let id_binding = user.id();
    cache_user_in_session(user).await?;
    Ok((Response::NewUser(username), id_binding))
}

async fn assign_rol_as_user(user_id: i32) -> Result<(), ApiError> {
    add_rol_to_user_service(user_id, Rol::User).await
}

pub async fn get_all_users_service() -> Result<Json<Vec<UserDto>>, ApiError> {
    let users = query_users().await?;
    Ok(into_json(users))
}

pub async fn find_user_service(user: FindUserDto) -> Result<Json<UserDto>, ApiError> {
    let user = load_user_by_username(&user.username).await?;
    Ok(Json(user.into()))
}

pub async fn login_service(login: LoginDto) -> Result<(Response, i32), ApiError> {
    let start = SystemTime::now();
    dbg!(format!("Start login service call = {:?}", start));
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
        let username = current_user.username();
        let binding_id = current_user.id();
        cache_user_in_session(current_user).await?;
        return Ok((Response::UserLogIn(username), binding_id));
    }
    Err(ApiError::IncorrectPassword)
}

pub async fn find_user_by_id_service(user_id: i32) -> Result<Json<UserDto>, ApiError> {
    let user = load_user_by_id(user_id).await?;
    Ok(Json(user.into()))
}

pub async fn find_user_by_username_service(username: &String) -> Result<User, ApiError> {
    Ok(load_user_by_username(username).await?)
}

fn into_json(users: Vec<User>) -> Json<Vec<UserDto>> {
    Json(users.into_iter().map(|user| user.into()).collect())
}

async fn cache_user_in_session(user: User) -> Result<(), ApiError> {
    let current_user = create_current_user(user).await?;
    Ok(save_user_session(&current_user).await?)
}

pub async fn patch_user_service(
    user_id: i32,
    profile: UpdateUserDto,
) -> Result<Response, ApiError> {
    let old_profile = load_user_by_id(user_id).await?;
    let new_name = match profile.name {
        Some(value) => Some(value),
        None => old_profile.name(),
    };
    let new_surname = match profile.surname {
        Some(value) => Some(value),
        None => old_profile.surname(),
    };
    let new_email = match profile.email {
        Some(value) => Some(value),
        None => old_profile.email(),
    };
    let update_time = Some(now());
    let update_profile = UpdateUser {
        password: None,
        name: new_name,
        surname: new_surname,
        email: new_email,
        update_at: update_time,
    };
    patch_user_record(user_id, update_profile).await?;
    Ok(Response::UpdateRecord)
}
