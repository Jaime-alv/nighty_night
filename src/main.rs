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
use dotenvy::dotenv;
use tracing::info;

use crate::{app::shutdown_signal, configuration::settings::Setting, core_checks::checking_status};

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

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => (),
        Err(error) => panic!("{error}"),
    };

    let app = create_app_route().await;

    match checking_status().await {
        Ok(msg) => info!("{msg}"),
        Err(error) => {
            tracing::error!("{error}");
            panic!("Shutting down.")
        }
    }

    let host = Setting::Host.get();
    // run it with hyper on localhost:3000
    info!("Start server, listening on {host}");

    axum::Server::bind(&host.parse().expect("Something went wrong with the address"))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

mod core_checks {

    use tracing::{debug, error, info};

    use crate::repository::{connection_redis::ping_redis, connection_psql::check_db_status};

    pub(super) async fn checking_status() -> Result<&'static str, &'static str> {
        let mut status = Vec::<bool>::new();

        info!("Checking databases connection...");

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

        debug!("PostgreSQL status...");
        match check_db_status() {
            Ok(msg) => {
                status.push(true);
                debug!("{msg}")
            }
            Err(error) => {
                status.push(false);
                error!("{error}")
            }
        };

        match !status.iter().any(|checks| checks.eq(&false)) {
            true => Ok("All checks are OK."),
            false => Err("Something went wrong."),
        }
    }
}
