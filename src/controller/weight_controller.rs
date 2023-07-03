use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::weight_dto::NewWeightDto,
    model::session_model::CurrentUser,
    service::{
        response_service::{forbidden, login_required},
        session_service::has_baby,
        weight_service::{get_weights_service, post_weight_service},
    },
};

pub(super) fn route_weight() -> Router {
    Router::new().route("/:baby_id/weights", get(get_weights).post(post_weight))
}

async fn get_weights(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    if auth.is_anonymous() {
        Err(login_required())
    } else if has_baby(auth, baby_id) {
        match get_weights_service(baby_id).await {
            Ok(value) => Ok(Json(value)),
            Err(error) => Err(error),
        }
    } else {
        Err(forbidden())
    }
}

async fn post_weight(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_measure): Json<NewWeightDto>,
) -> impl IntoResponse {
    if auth.is_anonymous() {
        Err(login_required())
    } else if has_baby(auth, baby_id) {
        post_weight_service(new_measure, baby_id).await
    } else {
        Err(forbidden())
    }
}
