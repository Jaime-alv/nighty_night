use redis::RedisError;

use crate::data::session_dto::CurrentUserDto;

use super::connection_redis::poll;

pub async fn set_user(key: &str, user: CurrentUserDto, duration: usize) -> Result<(), RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::pipe()
        .set(key, serde_json::to_string(&user)?)
        .expire(key, duration)
        .query_async(&mut conn)
        .await
}

pub async fn get_user(key: &str) -> Result<String, RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::cmd("GET").arg(key).query_async(&mut conn).await
}

pub async fn delete_user_session(key: &str) -> Result<(), RedisError> {
    let mut conn = poll().await.get_async_connection().await?;
    redis::cmd("DEL").arg(key).query_async(&mut conn).await
}
#[cfg(test)]
mod redis_test {
    use dotenvy::dotenv;

    use crate::repository::session_repository::get_user;

    fn set_env() {
        dotenv().ok();
    }

    #[tokio::test]
    async fn test_connection() {
        set_env();
        assert_eq!(get_user("user_1").await.unwrap(), "{\"id\":1,\"anonymous\":true,\"username\":\"guest\",\"roles\":[2],\"active\":true,\"baby_id\":[]}".to_string())
    }
}
