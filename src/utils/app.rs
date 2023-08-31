use axum::response::IntoResponse;
use tokio::signal;
use tracing::{info, debug, error};

use crate::{service::util_service::not_found, configuration::settings::Setting, connection::{connection_redis::ping_redis, connection_psql::check_db_status}};

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c=> {},

    }
}

pub async fn error_404() -> impl IntoResponse {
    not_found()
}

pub async fn checking_status() -> Result<&'static str, &'static str> {
    let mut status = Vec::<bool>::new();

    info!("Branch mode: {}", Setting::Branch.get());

    info!("Checking databases connection...");

    debug!("PING");
    match ping_redis().await {
        Ok(msg) => {
            status.push(true);
            debug!("{msg}")
        }
        Err(error) => {
            status.push(false);
            error!("{}", Setting::RedisHost.get());
            error!("Redis: {error}")
        }
    }

    debug!("PostgreSQL status...");
    match check_db_status() {
        Ok(msg) => {
            status.push(true);
            debug!("{msg}")
        }
        Err(error) => {
            status.push(false);
            error!("{}", Setting::DatabaseUrl.get());
            error!("{error}")
        }
    };

    match !status.iter().any(|checks| checks.eq(&false)) {
        true => Ok("All checks are OK."),
        false => Err("Something went wrong."),
    }
}