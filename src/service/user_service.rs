use chrono::Days;

use crate::{
    configuration::constant::GlobalCte,
    data::{
        common_structure::{AdminUserDto, SessionDto, UserDto},
        query_dto::Pagination,
        traits::Mandatory,
        user_dto::{FindUserDto, LoginDto, NewUserDto, UpdateUser, UpdateUserDto},
    },
    model::{role_model::Rol, user_model::User},
    repository::user_repository::{
        update_active_for_user, insert_new_user, delete_user,
        delete_all_users, select_user_by_id, select_user_by_username, update_user,
        select_all_users, select_id_from_username,
    },
    response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    },
    utils::{
        datetime::now,
        validator::{valid_password, validate_fields},
    },
};

use super::session_service::{create_current_user, save_user_session};

pub async fn post_new_user_service(
    new_user: NewUserDto,
) -> Result<(RecordResponse<SessionDto>, i32), ApiError> {
    if validate_fields(&new_user.data()) {
        return Err(ApiError::EmptyBody);
    }
    if valid_password(&new_user.password) {
        return Err(ApiError::Generic400Error("Password too short.".into()));
    }
    let user = match insert_new_user(new_user, Rol::User.into()) {
        Ok(user) => user,
        Err(_) => return Err(ApiError::DuplicateUser),
    };
    let id_binding = user.id();
    let new_user = cache_user_in_session(user).await?;
    Ok((RecordResponse::new(new_user), id_binding))
}

pub async fn get_all_users_service(
    pagination: Pagination,
) -> Result<PagedResponse<Vec<AdminUserDto>>, ApiError> {
    let current = pagination.page();
    let (users, total_pages) = select_all_users(pagination)?;
    let users: Vec<AdminUserDto> = users.into_iter().map(|user| user.into()).collect();
    let response = PagedResponse::new(users, current, total_pages);
    Ok(response)
}

pub async fn post_find_user_service(user: FindUserDto) -> Result<RecordResponse<UserDto>, ApiError> {
    let user = match select_user_by_username(&user.username) {
        Ok(value) => value,
        Err(_) => return Err(ApiError::NoUser),
    };
    let response = RecordResponse::new(user.into());
    Ok(response)
}

pub async fn login_service(login: LoginDto) -> Result<(RecordResponse<SessionDto>, i32), ApiError> {
    if validate_fields(&login.data()) {
        return Err(ApiError::EmptyBody);
    }
    let current_user = match select_user_by_username(&login.username) {
        Ok(u) => u,
        Err(_) => return Err(ApiError::IncorrectPassword),
    };
    if !current_user.active() {
        return Err(ApiError::NoActiveUser);
    }
    if current_user.is_password_match(&login.password) {
        let binding_id = current_user.id();
        let login_user: SessionDto = cache_user_in_session(current_user).await?;
        let dto = RecordResponse::new(login_user);
        return Ok((dto, binding_id));
    }
    Err(ApiError::IncorrectPassword)
}

pub async fn get_user_by_id_service(user_id: i32) -> Result<RecordResponse<UserDto>, ApiError> {
    let user = select_user_by_id(user_id)?;
    let response = RecordResponse::new(user.into());
    Ok(response)
}

async fn cache_user_in_session(user: User) -> Result<SessionDto, ApiError> {
    let current_user = create_current_user(user).await?;
    save_user_session(&current_user, None).await?;
    let user_dto: SessionDto = current_user.into();
    Ok(user_dto)
}

pub async fn patch_user_service(
    user_id: i32,
    profile: UpdateUserDto,
) -> Result<MsgResponse, ApiError> {
    let old_profile = select_user_by_id(user_id)?;
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
    update_user(user_id, update_profile)?;
    Ok(MsgResponse::UpdateRecord)
}

pub async fn delete_active_user_service(
    user_id: i32,
    active: bool,
) -> Result<MsgResponse, ApiError> {
    let time = now();
    update_active_for_user(user_id, active, time)?;
    Ok(MsgResponse::ActiveStatusUpdate)
}

/// User must be active = false, last updated 180 days ago (as per DeleteAccount const)
/// and user must not be current user.
pub async fn delete_user_service(user_id: i32, current_user: i32) -> Result<MsgResponse, ApiError> {
    let user = select_user_by_id(user_id)?;
    let inactive_period: i64 = GlobalCte::DeleteAccount.get().into();
    let inactive_time = (now() - user.updated_at().unwrap_or_default())
        .num_days()
        .ge(&inactive_period);
    if current_user.ne(&user_id) && inactive_time && !user.active() {
        delete_user(user_id)?;
        Ok(MsgResponse::DeleteRecord)
    } else {
        Err(ApiError::Forbidden)
    }
}

pub async fn delete_old_users_service() -> Result<MsgResponse, ApiError> {
    let inactive_period: u64 = GlobalCte::DeleteAccount.get().into();
    let older_than = now().checked_sub_days(Days::new(inactive_period)).unwrap();
    let rows = delete_all_users(older_than)?;
    Ok(MsgResponse::DeleteXRecords(rows))
}

/// Return user id if user with username exits.
pub async fn find_user_id_from_username(username: &str) -> Result<i32, ApiError> {
    match select_id_from_username(username) {
        Ok(id) => Ok(id),
        Err(_) => Err(ApiError::NoUser),
    }
}
