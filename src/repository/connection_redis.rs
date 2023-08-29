use axum_session::{Key, SecurityMode, SessionConfig};
use axum_session_auth::AuthConfig;
use redis::{Client, RedisError};

use crate::configuration::{settings::Setting, constant::GlobalCte};

pub async fn poll() -> Client {
    let address = Setting::RedisHost.get();
    Client::open(address).expect("Can't connect to redis")
}

pub async fn ping_redis() -> Result<String, RedisError> {
    let mut conn = match poll().await.get_connection(){
        Ok(server) => server,
        Err(error) => return Err(error)
    };
    redis::cmd("PING").query::<String>(&mut conn)
}

/// This Defaults as normal Cookies.
pub fn session_config() -> SessionConfig {
    SessionConfig::default()
}

/// Private cookies.
///
/// To enable private cookies for confidentiality, integrity, and authenticity.
/// When a Key is set it will automatically set the Cookie into an encrypted
/// Private cookie which both protects the cookies data from prying eye’s it
/// also ensures the authenticity of the cookie.
pub fn private_cookies_session() -> SessionConfig {
    SessionConfig::default()
        // 'Key::generate()' will generate a new key each restart of the server.
        // If you want it to be more permanent then generate and set it to a config file.
        // If with_key() is used it will set all cookies as private, which guarantees integrity, and authenticity.
        .with_key(Key::generate())
        // This is how we would Set a Database Key to encrypt as store our per session keys.
        // This MUST be set in order to use SecurityMode::PerSession.
        .with_database_key(Key::generate())
        // This is How you will enable PerSession SessionID Private Cookie Encryption. When enabled it will
        // Encrypt the SessionID and Storage with an Encryption key generated and stored per session.
        // This allows for Key renewing without needing to force the entire Session from being destroyed.
        // This Also helps prevent impersonation attempts.
        .with_security_mode(SecurityMode::PerSession)
}

pub fn auth_config() -> AuthConfig<i64> {
    let id: i64 = GlobalCte::DefaultAnonymousID.get().into();
    AuthConfig::<i64>::default().with_anonymous_user_id(Some(id))
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