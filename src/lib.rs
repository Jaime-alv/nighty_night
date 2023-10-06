// Copyright 2023 Jaime Alvarez Fernandez

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::Path;
use tracing::info;

use crate::{
    app::{expand_router_layer, create_router},
    configuration::settings::Setting,
    utils::app::{checking_status, set_anonymous_user, shutdown_signal},
};

pub mod app;
pub mod configuration;
mod connection;
mod controller;
pub mod data;
mod mapping;
pub mod model;
mod repository;
pub mod response;
mod schema;
mod security;
pub mod service;
pub mod utils;

/// Prepare environment variables.
///
/// If no env variables are set, defaults to local file
/// in path `/key/local.env`
pub fn set_environment() {
    if Setting::Branch.get().eq("local") {
        match dotenvy::from_path(Path::new("./key/local.env")) {
            Ok(_) => (),
            Err(error) => panic!("{error}"),
        }
    }
}

/// Check whether databases are ready or not.
pub async fn check_db_initialisation() {
    match checking_status().await {
        Ok(_) => (),
        Err(error) => panic!("{error}"),
    };

    match set_anonymous_user().await {
        Ok(_) => (),
        Err(error) => panic!("{error}"),
    };
}

/// Launch server
pub async fn serve_app() {
    let router = create_router();
    let app = expand_router_layer(router).await;

    info!("Branch mode: {}", Setting::Branch.get());

    let host = Setting::Host.get();
    // run it with hyper on localhost:3000
    info!("Start server, listening on {host}");

    axum::Server::bind(&host.parse().expect("Something went wrong with the address"))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
