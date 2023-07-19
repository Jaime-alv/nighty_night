use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::{query_dto::IdDto, weight_dto::InputWeightDto},
    model::session_model::CurrentUser,
    service::{
        session_service::authorize_and_has_baby,
        weight_service::{
            delete_weight_service, get_weights_service, patch_weight_service, post_weight_service,
        },
    },
};

pub(super) fn route_weight() -> Router {
    Router::new().route(
        "/:baby_id/weights",
        get(get_weights)
            .post(post_weight)
            .patch(patch_weight)
            .delete(delete_weight),
    )
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
    Json(new_measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    post_weight_service(new_measure, baby_id).await
}

async fn patch_weight(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record: Query<IdDto>,
    Json(measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    patch_weight_service(measure, record.id(), baby_id).await
}

async fn delete_weight(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    delete_weight_service(record_id.id(), baby_id).await
}
