use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::dream_dto::NewDreamDto,
    model::session_model::CurrentUser,
    service::{
        dream_service::{
            dream_summary_service, filter_dreams_by_date_service, get_all_dreams_from_baby_service,
            post_dream_service,
        },
        response_service::{empty_query, forbidden},
        session_service::has_baby,
    },
};

pub(super) fn route_dream() -> Router {
    Router::new()
        .route("/:baby_id/dreams", get(get_dreams).post(post_dream))
        .route("/:baby_id/dreams/summary", get(dream_summary))
}

async fn get_dreams(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match date.get("date") {
            Some(d) => match filter_dreams_by_date_service(baby_id, d).await {
                Ok(dreams) => Ok(Json(dreams)),
                Err(error) => Err(error),
            },
            None => match get_all_dreams_from_baby_service(baby_id).await {
                Ok(dreams) => Ok(Json(dreams)),
                Err(error) => Err(error),
            },
        }
    } else {
        Err(forbidden())
    }
}

async fn post_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_dream): Json<NewDreamDto>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        post_dream_service(new_dream, baby_id).await
    } else {
        Err(forbidden())
    }
}

async fn dream_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match date.get("date") {
            Some(string_date) => match dream_summary_service(baby_id, string_date).await {
                Ok(dreams) => Ok(Json(dreams)),
                Err(error) => Err(error),
            },
            None => Err(empty_query()),
        }
    } else {
        Err(forbidden())
    }
}
