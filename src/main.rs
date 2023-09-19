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

use app::create_app_route;
use std::path::Path;
use tracing::info;

use crate::{
    configuration::settings::Setting,
    utils::app::{checking_status, set_anonymous_user, shutdown_signal},
};

mod app;
mod configuration;
mod connection;
mod controller;
mod data;
mod mapping;
mod model;
mod repository;
mod response;
mod schema;
mod security;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    if Setting::Branch.get().eq("local") {
        match dotenvy::from_path(Path::new("./key/local.env")) {
            Ok(_) => (),
            Err(error) => panic!("{error}"),
        }
    };

    match checking_status().await {
        Ok(_) => (),
        Err(error) => panic!("{error}"),
    };

    match set_anonymous_user().await {
        Ok(_) => (),
        Err(error) => panic!("{error}"),
    };

    let app = create_app_route().await;

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
