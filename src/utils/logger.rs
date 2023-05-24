use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::configuration::settings::Setting;

pub fn setup_logger() {
    let level = set_level();
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn set_level() -> Level {
    let binding = Setting::LoggerLevel.get();
    let input = binding.as_str();
    match input {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "trace" => Level::TRACE,
        _ => Level::DEBUG,
    }
}