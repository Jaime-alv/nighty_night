use diesel::pg::PgConnection;
use diesel::prelude::*;
use nighty_night::repository::admin_repository::select_stats_from_tables;
use nighty_night::{
    configuration::settings::Setting,
    connection::connection_redis::poll,
    data::session_dto::CurrentUserDto,
    repository::session_repository::{insert_user_session, select_user_session},
};
use redis::RedisError;

pub mod common;

#[ctor::ctor]
fn init() {
    common::initialiser::init()
}

#[tokio::test]
async fn test_redis_database() {
    assert_eq!(
        ping_redis()
            .await
            .expect("Redis should be started")
            .as_str(),
        "PONG"
    );
}

#[test]
fn test_postgresql_database() {
    assert!(
        check_db_status().is_ok(),
        "PostgreSQL should start => {}",
        check_db_status().unwrap_err()
    );

    let response_stats = select_stats_from_tables();
    assert!(
        response_stats.is_ok(),
        "Should load miscellaneous data from PostgreSQL"
    );
    let data = response_stats.unwrap();

    assert!(
        data.users.value.ge(&2),
        "There should be more than two users. There are {}",
        data.users.value
    );
}

async fn ping_redis<'a>() -> Result<String, RedisError> {
    let mut conn = match poll().await.get_connection() {
        Ok(server) => server,
        Err(error) => return Err(error),
    };
    redis::cmd("PING").query::<String>(&mut conn)
}

fn check_db_status<'a>() -> Result<String, String> {
    let database_url = Setting::DatabaseUrl.get();
    match PgConnection::establish(&database_url) {
        Ok(_) => Ok("PostgreSQL ready.".to_string()),
        Err(error) => Err(format!("PostgreSQL: {error}")),
    }
}

#[test]
fn test_branch() {
    assert_eq!(Setting::Branch.get(), "local");
    assert_eq!(Setting::RedisHost.get(), "redis://127.0.0.1:6379/");
}

#[tokio::test]
async fn test_connection() {
    let redis_key = "user_1";
    let guest_user = CurrentUserDto {
        id: 1,
        anonymous: true,
        username: "guest".to_string(),
        roles: vec![2],
        active: true,
        baby_id: vec![],
    };

    let response_insert_user = insert_user_session(redis_key, guest_user, 60).await;

    assert!(
        response_insert_user.is_ok(),
        "Should create a user in Redis"
    );

    let result_redis_connection = select_user_session(redis_key).await;
    assert!(
        result_redis_connection.is_ok(),
        "Should connect to redis and find a user"
    );
    let guest = result_redis_connection.unwrap();
    assert_eq!(guest, "{\"id\":1,\"anonymous\":true,\"username\":\"guest\",\"roles\":[2],\"active\":true,\"baby_id\":[]}".to_string())
}
