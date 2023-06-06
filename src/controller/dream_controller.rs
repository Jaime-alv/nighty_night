use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::dream_dto::NewDreamDto,
    model::session_model::CurrentUser,
    service::{
        dream_service::{get_all_dreams_from_baby_service, post_dream_service},
        response_service::forbidden,
        session_service::has_baby,
    },
};

pub(super) fn route_dream() -> Router {
    Router::new().route("/:baby_id/dreams", get(get_dreams).post(post_dream))
}

async fn get_dreams(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match get_all_dreams_from_baby_service(baby_id).await {
            Ok(dreams) => Ok(Json(dreams)),
            Err(error) => Err(error),
        }
    } else {
        Err(forbidden().await)
    }
}

async fn post_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_dream): Json<NewDreamDto>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match post_dream_service(new_dream, baby_id).await {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    } else {
        Err(forbidden().await)
    }
}
