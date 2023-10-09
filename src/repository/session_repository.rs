use redis::RedisError;

use crate::{connection::connection_redis::poll, data::session_dto::CurrentUserDto};

pub async fn insert_user_session(
    key: &str,
    user: CurrentUserDto,
    duration_in_seconds: usize,
) -> Result<(), RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::pipe()
        .set(key, serde_json::to_string(&user)?)
        .expire(key, duration_in_seconds)
        .query_async(&mut conn)
        .await
}

pub async fn insert_user_session_indefinitely(
    key: &str,
    user: CurrentUserDto,
) -> Result<(), RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::pipe()
        .set(key, serde_json::to_string(&user)?)
        .query_async(&mut conn)
        .await
}

pub async fn select_user_session(key: &str) -> Result<String, RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::cmd("GET").arg(key).query_async(&mut conn).await
}

pub async fn delete_user_session(key: &str) -> Result<(), RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::cmd("DEL").arg(key).query_async(&mut conn).await
}

pub async fn select_user_session_exists(key: &str) -> Result<bool, RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::cmd("EXISTS").arg(key).query_async(&mut conn).await
}