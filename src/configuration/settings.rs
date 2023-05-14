use std::env;

use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Server {
    pub address: String,
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub uri: String,
}

pub struct Settings {
    pub database_url: Database,
    pub logger_level: Logger,
    pub server: Server,
}

pub fn set_database_url() -> String {
    let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    uri
}

pub fn set_logger_level() -> String {
    env::var("LOGGER_LEVEL").expect("LOGGER_LEVEL must be set")
}

pub fn set_server() -> String {
    let address = env::var("ADDRESS").expect("ADDRESS must be set");
    let port = env::var("PORT").expect("PORT must be set");
    format!("{address}:{port}")
}

pub fn check_env_file() {
    match dotenv() {
        Ok(_) => (),
        Err(_) => panic!("No .env file"),
    };
}
