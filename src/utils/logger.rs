use std::env;


use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::configuration::settings::set_logger_level;

pub fn setup_logger() {
    let binding = set_logger_level();
    let input = binding.as_str();
    let level = match  input {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "trace" => Level::TRACE,
        _ => Level::DEBUG
    };
    let env = format!("webapp_test={level},tower_http={level}");
    env::set_var("RUST_LOG", env);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
