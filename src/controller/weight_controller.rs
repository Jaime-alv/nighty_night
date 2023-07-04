use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::weight_dto::NewWeightDto,
    model::session_model::CurrentUser,
    service::{
        session_service::authorize_and_has_baby,
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
    authorize_and_has_baby(auth, baby_id)?;
    get_weights_service(baby_id).await
}

async fn post_weight(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_measure): Json<NewWeightDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    post_weight_service(new_measure, baby_id).await
}
