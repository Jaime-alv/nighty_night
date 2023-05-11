use app::create_app_route;

mod app;
mod controller;
mod data;
mod model;
mod repository;
mod schema;
mod service;
mod error;
mod mapping;
mod security;

#[tokio::main]
async fn main() {
    let app = create_app_route();
    // TODO: Add logger

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
