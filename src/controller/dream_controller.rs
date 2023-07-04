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
        session_service::authorize_and_has_baby,
        util_service::parse_query_field,
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
    authorize_and_has_baby(auth, baby_id)?;
    match date.get("date") {
        Some(d) => filter_dreams_by_date_service(baby_id, d).await,
        None => get_all_dreams_from_baby_service(baby_id).await,
    }
}

async fn post_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_dream): Json<NewDreamDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    post_dream_service(new_dream, baby_id).await
}

async fn dream_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let string_date = parse_query_field(date, "date")?;
    dream_summary_service(baby_id, &string_date).await
}
