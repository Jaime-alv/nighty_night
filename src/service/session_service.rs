use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    configuration::settings::Setting,
    data::session_dto::CurrentUserDto,
    error::error::ApiError,
    mapping::rol_mapper::translate_roles,
    model::{role_model::Rol, session_model::CurrentUser},
    repository::{
        session_repository::{exists_user, get_user, set_user},
        user_repository::{find_babies_id, find_roles_id, load_user_by_id},
    },
};

pub async fn login_session<T>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> Result<(), ApiError>
where
    i32: From<T>,
    <T as TryInto<i32>>::Error: std::fmt::Debug,
    i64: From<T>,
    T: std::marker::Copy,
{
    let id: i32 = user_id.try_into().clone().unwrap();
    let user = read_from_db(id).await?;
    save_user_session(&user).await?;
    auth.login_user(user_id.into());
    Ok(())
}

pub async fn save_user_session(user: &CurrentUser) -> Result<(), ApiError> {
    let current_user_dto = CurrentUserDto::new(
        user.id(),
        user.anonymous(),
        user.username(),
        user.roles_id(),
        user.active(),
        user.baby_id(),
    );
    let key = user_redis_key(user.id());
    let duration = Setting::SessionDuration
        .get()
        .parse::<usize>()
        .unwrap_or(600);
    set_user(&key, current_user_dto, duration).await?;
    Ok(())
}

pub async fn update_user_session(user: &CurrentUser) -> Result<(), ApiError> {
    let update_user = read_from_db(user.id().try_into().unwrap()).await?;
    save_user_session(&update_user).await
}

pub async fn load_user_session(id: i64) -> Result<CurrentUser, ApiError> {
    let key = user_redis_key(id);
    let string_user = get_user(&key).await?;
    let user: CurrentUserDto = match serde_json::from_str(&string_user) {
        Ok(user) => user,
        Err(err) => return Err(ApiError::Redis(err.into())),
    };
    Ok(CurrentUser::new(
        user.id,
        user.anonymous,
        user.username,
        translate_roles(&user.roles),
        user.active,
        user.baby_id,
    ))
}

pub async fn read_from_db(user: i32) -> Result<CurrentUser, ApiError> {
    let current_user = load_user_by_id(user).await?;
    let roles: Vec<u8> = find_roles_id(current_user.id()).await.into_iter().collect();
    let translate_roles: Vec<Rol> = translate_roles(&roles);

    let babies: Vec<i32> = find_babies_id(current_user.id()).await;

    let user_session = CurrentUser::new(
        current_user.id().into(),
        translate_roles.contains(&Rol::Anonymous),
        current_user.username(),
        translate_roles,
        current_user.active(),
        babies,
    );
    Ok(user_session)
}

pub async fn user_session_exists(id: i64) -> bool {
    let key = user_redis_key(id);
    exists_user(&key).await.unwrap()
}

fn user_redis_key(id: i64) -> String {
    format!("user_{}", id)
}

/// Check if user has admin privileges
pub fn current_user_is_admin(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> Result<(), ApiError> {
    match auth.current_user.unwrap().is_admin() {
        true => Ok(()),
        false => Err(ApiError::Forbidden),
    }
}

fn has_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    baby_id: i32,
) -> bool {
    let babies = auth.current_user.unwrap().baby_id();
    babies.contains(&baby_id)
}

/// Check if user is authenticated and baby has a relationship with user.
pub fn authorize_and_has_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    baby_id: i32,
) -> Result<(), ApiError> {
    if auth.is_anonymous() {
        return Err(ApiError::LoginRequired);
    } else if has_baby(auth, baby_id) {
        Ok(())
    } else {
        Err(ApiError::Forbidden)
    }
}

pub fn login_required(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>
) -> Result<(), ApiError> {
    match auth.is_authenticated() {
        true => Ok(()),
        false => Err(ApiError::LoginRequired),
    }
}