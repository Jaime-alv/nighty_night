use axum_session::{Key, SecurityMode, SessionConfig};
use axum_session_auth::AuthConfig;

use super::constant::GlobalCte;

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
