use app::create_app_route;
use configuration::settings::{check_env_file, set_server};

use crate::app::shutdown_signal;


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

    let host = set_server();
    // run it with hyper on localhost:3000

    tracing::info!("Start server, listening on {host}");
    
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
