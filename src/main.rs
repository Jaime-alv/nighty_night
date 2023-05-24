use app::create_app_route;
use configuration::settings::{check_env_file, server};
use tracing::info;

use crate::{app::shutdown_signal, core_checks::check_database_status};

mod app;
mod configuration;
mod controller;
mod data;
mod error;
mod mapping;
mod model;
mod repository;
mod schema;
mod security;
mod service;
mod utils;

// pub const APP_SETTING: Settings = Settings::new();

#[tokio::main]
async fn main() {
    check_env_file();

    let app = create_app_route().await;

    match check_database_status().await {
        Ok(msg) => info!("{msg}"),
        Err(error) => {
            tracing::error!("{error}");
            panic!("Shutting down.")
        }
    }

    let host = server();
    // run it with hyper on localhost:3000
    info!("Start server, listening on {host}");

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

mod core_checks {
    
    use tracing::{debug, error, info};

    use crate::{
        repository::{connection_redis::ping_redis, connection_sqlite::check_sqlite_status},
    };

    pub(super) async fn check_database_status() -> Result<&'static str, &'static str>{
        info!("Checking database connection.");

        let mut status = Vec::<bool>::new();

        debug!("PING");
        match ping_redis().await {
            Ok(msg) => {
                status.push(true);
                debug!("{msg}")
            }
            Err(error) => {
                status.push(false);
                error!("Redis: {error}")
            }
        }

        debug!("SQlite status...");
        match check_sqlite_status() {
            Ok(msg) => {
                status.push(true);
                debug!("{msg}")
            },
            Err(error) => {
                status.push(false);
                error!("SQLite: {error}")
            }
        };

        match !status.iter().any(|checks| checks.eq(&false)) {
            true => Ok("DBs ready."),
            false => Err("Check DB status."),
        }
    }
}
