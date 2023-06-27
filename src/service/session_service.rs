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

pub async fn login_session<T: Into<i64>>(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: T,
) -> () {
    auth.login_user(user_id.into())
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

pub async fn update_user_session(user: &CurrentUser) -> Result<(), ApiError> {
    let update_user = match read_from_db(user.id().try_into().unwrap()).await {
        Ok(user) => user,
        Err(error) => return Err(error),
    };
    save_user_session(&update_user).await
}

pub async fn load_user_session(id: i64) -> Result<CurrentUser, ApiError> {
    let key = user_redis_key(id);
    let string_user = get_user(&key).await.unwrap();
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
    match load_user_by_id(user).await {
        Ok(user) => {
            let roles: Vec<u8> = find_roles_id(user.id()).await.into_iter().collect();
            let translate_roles: Vec<Rol> = translate_roles(&roles);

            let babies: Vec<i32> = find_babies_id(user.id()).await;

            let user_session = CurrentUser::new(
                user.id().into(),
                translate_roles.contains(&Rol::Anonymous),
                user.username(),
                translate_roles,
                user.active(),
                babies,
            );
            Ok(user_session)
        }
        Err(error) => Err(ApiError::DBError(error)),
    }
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
    let babies = auth.current_user.unwrap().baby_id();
    babies.contains(&baby_id)
}
