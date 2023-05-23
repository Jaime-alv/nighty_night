use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;
use redis::RedisError;

use crate::{
    data::session_dto::CurrentUserDto,
    mapping::rol_mapper::translate_roles,
    model::session_model::CurrentUser,
    repository::session_repository::{get_user, set_user, exists_user}, configuration::settings::session_default_duration,
};

pub async fn login_session<T: Into<i64>>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> () {
    auth.login_user(user_id.into())
}

pub async fn save_user_session(user: &CurrentUser, roles: Vec<u8>) -> Result<(), RedisError> {
    let current_user_dto = CurrentUserDto::new(
        user.id(),
        user.anonymous(),
        user.username(),
        roles.into_iter().collect(),
        user.active()
    );
    let key = user_redis_key(user.id());
    let duration = session_default_duration();
    match set_user(&key, current_user_dto, duration).await {
        Ok(_) => Ok(()),
        Err(error) => {
            tracing::error!("{error}");
            Err(error)},
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
        translate_roles(&user.roles),
        user.active
    )
}

pub async fn user_session_exists(id: i64) -> bool {
    let key = user_redis_key(id);
    exists_user(&key).await.unwrap()
}

fn user_redis_key(id: i64) -> String {
    format!("user_{}", id)
}
