use std::env;

use dotenvy::dotenv;

pub fn database_url() -> String {
    let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    uri
}

pub fn logger_level() -> String {
    env::var("LOGGER_LEVEL").expect("LOGGER_LEVEL must be set")
}

pub fn server() -> String {
    let address = env::var("ADDRESS").expect("ADDRESS must be set");
    let port = env::var("PORT").expect("PORT must be set");
    format!("{address}:{port}")
}

pub fn redis_server() -> String {
    let address = env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS must be set");
    let port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    format!("redis://{address}:{port}/") 
}

pub fn branch() -> String {
    env::var("BRANCH").expect("BRANCH must be set")
}

pub fn session_default_duration() -> usize {
    match env::var("SESSION_DURATION").expect("SESSION_DURATION must be set").parse::<usize>() {
        Ok(duration) => duration,
        Err(_) => 3600,
    }
}

pub fn check_env_file() {
    match dotenv() {
        Ok(_) => (),
        Err(_) => panic!("No .env file"),
    };
}
