use axum::{extract::MatchedPath, Router};

use axum_session::{SessionLayer, SessionRedisPool, SessionStore};
use axum_session_auth::AuthSessionLayer;
use controller::{baby_controller::route_baby, user_controller::route_user};
use hyper::Request;
use tower_http::trace::TraceLayer;
use tracing::{error, info_span};

use crate::{
    configuration::settings::Setting,
    controller,
    model::session_model::CurrentUser,
    repository::connection_redis::{auth_config, poll, private_cookies_session, session_config},
    utils::{app::error_404, logger::setup_logger},
};

/// Create app object with routes and layers.
/// Session layer must be on top of session auth layer.
pub(super) async fn create_app_route() -> Router {
    setup_logger();

    let config = if Setting::Branch.get().eq("local") {
        session_config()
    } else {
        private_cookies_session()
    };
    let poll = poll().await;
    let session_store = SessionStore::<SessionRedisPool>::new(Some(poll.clone().into()), config);

    //Create the Database table for storing our Session Data.
    match session_store.initiate().await {
        Ok(_) => (),
        Err(error) => error!("{error}"),
    };

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
        )
        .layer(
            AuthSessionLayer::<CurrentUser, i64, SessionRedisPool, redis::Client>::new(Some(poll))
                .with_config(auth_config()),
        )
        .layer(SessionLayer::new(session_store))
        .fallback(error_404);
    app
}
