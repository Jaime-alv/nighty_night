use std::env;

pub enum Setting {
    Host,
    Branch,
    DatabaseUrl,
    LoggerLevel,
    RedisHost,
    SessionDuration,
}

impl Setting {
    pub fn get(&self) -> String {
        match self {
            Setting::Host => {
                let address = read_environment_key("ADDRESS");
                let port = read_environment_key("PORT");
                format!("{address}:{port}")
            }
            Setting::Branch => env::var("BRANCH").unwrap_or("local".to_string()),
            Setting::DatabaseUrl => {
                let user = read_environment_key("POSTGRES_USER");
                let password = read_environment_key("POSTGRES_PASSWORD");
                let host = read_environment_key("POSTGRES_HOST");
                let port = read_environment_key("POSTGRES_PORT");
                let db = read_environment_key("POSTGRES_DB");
                format!("postgres://{user}:{password}@{host}:{port}/{db}")
            }
            Setting::LoggerLevel => read_environment_key("LOGGER_LEVEL"),
            Setting::RedisHost => {
                let address = read_environment_key("REDIS_ADDRESS");
                let port = read_environment_key("REDIS_PORT");
                format!("redis://{address}:{port}/")
            }
            Setting::SessionDuration => read_environment_key("SESSION_DURATION"),
        }
    }
}

fn read_environment_key(key: &str) -> String {
    match env::var(key) {
        Ok(var) => var,
        Err(error) => {
            tracing::error!("{error}: {key} must be set");
            return format!("{key} must be set");
        }
    }
}
