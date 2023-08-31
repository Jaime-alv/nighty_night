use redis::{Client, RedisError};

use crate::configuration::settings::Setting;

pub async fn poll() -> Client {
    let address = Setting::RedisHost.get();
    Client::open(address).expect("Can't connect to redis")
}

pub async fn ping_redis() -> Result<String, RedisError> {
    let mut conn = match poll().await.get_connection() {
        Ok(server) => server,
        Err(error) => return Err(error),
    };
    redis::cmd("PING").query::<String>(&mut conn)
}

#[cfg(test)]
mod test_redis_connection {
    use super::ping_redis;

    use dotenvy::dotenv;

    fn set_env() {
        dotenv().ok();
    }

    #[tokio::test]
    async fn test_ping() {
        set_env();
        assert_eq!(ping_redis().await.unwrap(), "PONG".to_string());
    }
}
