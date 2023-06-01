use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{error::error::ApiError, model::session_model::CurrentUser};

pub async fn forbidden() -> ApiError {
    ApiError::Forbidden
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
    auth.current_user.unwrap().has_baby(baby_id)
}
