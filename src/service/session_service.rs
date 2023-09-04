use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;
use uuid::Uuid;

use crate::{
    configuration::settings::Setting,
    data::session_dto::CurrentUserDto,
    mapping::rol_mapper::translate_roles,
    model::{role_model::Rol, session_model::CurrentUser, user_model::User},
    repository::{
        baby_repository::get_baby_id_from_unique_id,
        session_repository::{delete_user_session, get_user, set_user, set_user_indefinitely},
        user_repository::{find_babies_unique_id, find_roles_id, load_user_by_id},
    },
    response::{error::ApiError, response::MsgResponse},
};

pub async fn login_session<T>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> Result<(), ApiError>
where
    i32: From<T>,
    i64: From<T>,
{
    auth.login_user(user_id.into());
    Ok(())
}

pub async fn logout_session<T>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> Result<MsgResponse, ApiError>
where
    i32: From<T>,
    i64: From<T>,
{
    let key = user_redis_key(user_id.into());
    auth.logout_user();
    match delete_user_session(&key).await {
        Ok(_) => Ok(MsgResponse::LogoutUser),
        Err(e) => Err(ApiError::Redis(e)),
    }
}

pub async fn save_user_session(
    user: &CurrentUser,
    duration: Option<usize>,
) -> Result<(), ApiError> {
    let key = user_redis_key(user.id());
    let session_duration = match duration {
        Some(value) => value,
        None => Setting::SessionDuration
            .get()
            .parse::<usize>()
            .unwrap_or(600),
    };
    set_user(&key, (*user).clone().into(), session_duration).await?;
    Ok(())
}

pub async fn save_user_indefinitely(user: &CurrentUser) -> Result<(), ApiError> {
    let key = user_redis_key(user.id());
    set_user_indefinitely(&key, (*user).clone().into()).await?;
    Ok(())
}

pub async fn update_user_session(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> Result<(), ApiError> {
    let user_id: i32 = auth.id.try_into().unwrap();
    clear_cache_current_user(&auth);
    let update_user = read_user_from_db(user_id).await?;
    save_user_session(&update_user, None).await
}

pub async fn load_user_session(id: i64) -> Result<CurrentUser, ApiError> {
    let key = user_redis_key(id);
    let string_user = get_user(&key).await?;
    let user: CurrentUserDto = match serde_json::from_str(&string_user) {
        Ok(user) => user,
        Err(err) => return Err(ApiError::Redis(err.into())),
    };
    Ok(user.into())
}

pub async fn read_user_from_db(user: i32) -> Result<CurrentUser, ApiError> {
    let current_user = load_user_by_id(user)?;
    create_current_user(current_user).await
}

pub async fn create_current_user(current_user: User) -> Result<CurrentUser, ApiError> {
    let roles = find_roles_id(current_user.id())?;
    let babies = find_babies_unique_id(current_user.id())?
        .into_iter()
        .collect();
    let translate_roles: Vec<Rol> = translate_roles(&roles.into_iter().collect::<Vec<u8>>());

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
    baby_id: Uuid,
) -> bool {
    let babies = auth.current_user.unwrap().baby_id();
    babies.contains(&baby_id)
}

/// Check if user is authenticated and baby has a relationship with user.
pub fn authorize_and_has_baby_unique_id(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    baby_unique_id: Uuid,
) -> Result<i32, ApiError> {
    if auth.is_anonymous() {
        return Err(ApiError::LoginRequired);
    } else if has_baby(auth, baby_unique_id) {
        let id = get_baby_id_from_unique_id(baby_unique_id)?;
        Ok(id)
    } else {
        Err(ApiError::Forbidden)
    }
}

pub fn login_required(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> Result<(), ApiError> {
    match auth.is_authenticated() {
        true => Ok(()),
        false => Err(ApiError::LoginRequired),
    }
}

pub fn clear_cache_current_user(
    auth: &AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> () {
    auth.cache_clear_user(auth.id)
}
