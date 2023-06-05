use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    configuration::settings::Setting,
    data::session_dto::CurrentUserDto,
    error::error::ApiError,
    mapping::rol_mapper::translate_roles,
    model::session_model::CurrentUser,
    repository::{
        session_repository::{exists_user, get_user, set_user},
        user_repository::load_user_by_id,
    },
};

pub async fn login_session<T: Into<i64>>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> () {
    auth.login_user(user_id.into())
}

pub async fn save_user_session(user: &CurrentUser, roles: Vec<u8>) -> Result<(), ApiError> {
    let current_user_dto = CurrentUserDto::new(
        user.id(),
        user.anonymous(),
        user.username(),
        roles.into_iter().collect(),
        user.active(),
    );
    let key = user_redis_key(user.id());
    let duration = match Setting::SessionDuration.get().parse::<usize>() {
        Ok(time) => time,
        Err(_) => 600,
    };
    match set_user(&key, current_user_dto, duration).await {
        Ok(_) => Ok(()),
        Err(error) => {
            tracing::error!("{error}");
            Err(ApiError::Redis(error))
        }
    }
}

pub async fn load_user_session(id: i64) -> CurrentUser {
    let key = user_redis_key(id);
    let string_user = get_user(&key).await.unwrap();
    let user: CurrentUserDto = serde_json::from_str(&string_user).unwrap();
    CurrentUser::new(
        user.id,
        user.anonymous,
        user.username,
        translate_roles(&user.roles).await,
        user.active,
    )
}

pub async fn user_session_exists(id: i64) -> bool {
    let key = user_redis_key(id);
    exists_user(&key).await.unwrap()
}

fn user_redis_key(id: i64) -> String {
    format!("user_{}", id)
}

pub async fn is_admin(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> bool {
    auth.current_user.unwrap().is_admin()
}

pub async fn has_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    baby_id: i32,
) -> bool {
    let user_id: i32 = auth.id.try_into().unwrap();
    let user = load_user_by_id(user_id).unwrap();
    user.has_baby(baby_id)
}
