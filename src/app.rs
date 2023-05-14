use axum::Router;

use controller::{baby_controller::route_baby, user_controller::route_user};

use crate::{controller, utils::logger::setup_logger};

/// Create app object with routes and layers.
pub(super) async fn create_app_route() -> Router {
    setup_logger();
    let app = Router::new().nest(
        "/api",
        Router::new().merge(route_user()).merge(route_baby()),
    );
    app
}