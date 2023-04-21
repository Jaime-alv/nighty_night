mod controller;
mod model;
mod schema;

use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use diesel::{Connection, SqliteConnection};

use controller::{task_controller::*, user_controller::*};
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_app_route() -> Router {
    let app = Router::new()
        .nest("/api", route_user())
        .nest("/api", route_task());
    app
}

fn route_user() -> Router {
    let routes = Router::new()
        .route("/register", post(register_new_user))
        .route("/all", get(get_all_users))
        .route("/user", post(find_user));
    Router::new().nest("/auth", routes)
}

fn route_task() -> Router {
    let routes = Router::new()
        .route("/", get(get_all_tasks))
        .route("/:user_id/new", post(new_task))
        .route("/:user_id", get(get_task))
        .route("/:user_id/", get(get_task_by_id))
        ;
    Router::new().nest("/task", routes)
}
