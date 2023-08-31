use axum::response::IntoResponse;
use tokio::signal;
use tracing::{debug, error, info};

use crate::{
    configuration::{constant::GlobalCte, settings::Setting},
    connection::{connection_psql::check_db_status, connection_redis::ping_redis},
    response::error::ApiError,
    service::{
        session_service::{read_user_from_db, save_user_indefinitely},
        util_service::not_found,
    },
};

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

pub async fn set_anonymous_user() -> Result<(), ApiError> {
    let anonymous_id: i32 = GlobalCte::DefaultAnonymousID.get().try_into().unwrap();
    let user = read_user_from_db(anonymous_id).await?;
    save_user_indefinitely(&user).await
}
