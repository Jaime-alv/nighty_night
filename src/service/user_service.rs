use axum::Json;

use crate::{
    configuration::constant::GlobalCte,
    data::{
        query_dto::Pagination,
        traits::Mandatory,
        user_dto::{
            FindUserDto, LoginDto, NewUserDto, UpdateUser, UpdateUserDto, UserDto,
        }, admin_dto::AdminUserDto,
    },
    error::error::ApiError,
    model::{role_model::Rol, user_model::User},
    repository::user_repository::{
        alter_active_status_for_user, create_user, delete_user_from_db, load_user_by_id,
        load_user_by_username, patch_user_record, query_users,
    },
    utils::{
        datetime::now,
        response::Response,
        validator::{valid_password, validate_fields},
    },
};

use super::{
    session_service::{create_current_user, save_user_session},
    util_service::records_is_not_empty,
};

pub async fn create_user_service(new_user: NewUserDto) -> Result<(Response, i32), ApiError> {
    if validate_fields(&new_user.data()) {
        return Err(ApiError::EmptyBody);
    }
    if valid_password(&new_user.password) {
        return Err(ApiError::Generic400Error("Password too short.".into()));
    }
    let user = create_user(new_user, Rol::User.into())?;
    let username = user.username();
    let id_binding = user.id();
    cache_user_in_session(user).await?;
    Ok((Response::NewUser(username), id_binding))
}

pub async fn get_all_users_service(
    pagination: Pagination,
) -> Result<Json<Vec<AdminUserDto>>, ApiError> {
    let (users, _pages) = query_users(pagination)?;
    Ok(Json(
        records_is_not_empty(users)?
            .into_iter()
            .map(|user| user.into())
            .collect(),
    ))
}

pub async fn find_user_service(user: FindUserDto) -> Result<Json<UserDto>, ApiError> {
    let user = load_user_by_username(&user.username)?;
    Ok(Json(user.into()))
}

pub async fn login_service(login: LoginDto) -> Result<(Response, i32), ApiError> {
    if validate_fields(&login.data()) {
        return Err(ApiError::EmptyBody);
    }
    let current_user = match load_user_by_username(&login.username) {
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
    let user = load_user_by_id(user_id)?;
    Ok(Json(user.into()))
}

pub async fn find_user_by_username_service(username: &str) -> Result<User, ApiError> {
    match load_user_by_username(username) {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiError::NoUser),
    }
}

async fn cache_user_in_session(user: User) -> Result<(), ApiError> {
    let current_user = create_current_user(user).await?;
    Ok(save_user_session(&current_user).await?)
}

pub async fn patch_user_service(
    user_id: i32,
    profile: UpdateUserDto,
) -> Result<Response, ApiError> {
    let old_profile = load_user_by_id(user_id)?;
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
    patch_user_record(user_id, update_profile)?;
    Ok(Response::UpdateRecord)
}

pub async fn alter_active_user_service(user_id: i32, active: bool) -> Result<Response, ApiError> {
    let time = now();
    alter_active_status_for_user(user_id, active, time)?;
    Ok(Response::ActiveStatusUpdate)
}

/// User must be active = false and last updated 180 days ago (as per DeleteAccount const).
pub async fn delete_user_service(user_id: i32, current_user: i32) -> Result<Response, ApiError> {
    let user = load_user_by_id(user_id)?;
    let inactive_period: i64 = GlobalCte::DeleteAccount.get().into();
    let inactive_time = (now() - user.updated_at().unwrap_or_default())
        .num_days()
        .ge(&inactive_period);
    if current_user.ne(&user_id) && inactive_time && !user.active() {
        delete_user_from_db(user_id)?;
        Ok(Response::DeleteRecord)
    } else {
        Err(ApiError::Forbidden)
    }
}
