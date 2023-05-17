use axum::{extract::MatchedPath, Router};

use controller::{baby_controller::route_baby, user_controller::route_user};
use hyper::Request;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{controller, utils::logger::setup_logger};

/// Create app object with routes and layers.
pub(super) async fn create_app_route() -> Router {
    setup_logger();
    let app = Router::new()
        .nest(
            "/api",
            Router::new().merge(route_user()).merge(route_baby()),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );
    app
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    //#[cfg(not(unix))]
    //let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::debug!("signal received, starting graceful shutdown");
}
