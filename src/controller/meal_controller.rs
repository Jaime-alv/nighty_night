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
    data::meal_dto::NewMealDto,
    model::session_model::CurrentUser,
    service::{
        meal_service::{
            filter_meals_by_date_service, get_meals_service,
            post_meal_service,
        },
        session_service::authorize_and_has_baby,
        util_service::parse_query_field, meal_summary_service::{meal_summary_service, meal_summary_today_service, meal_summary_last_days_service, meal_summary_range_service},
    },
};

pub(super) fn route_meal() -> Router {
    Router::new()
        .route("/:baby_id/meals", get(get_meals).post(post_meal))
        .route("/:baby_id/meals/summary", get(meal_summary))
        .route("/:baby_id/meals/summary/today", get(meal_summary_today))
        .route("/:baby_id/meals/summary/last", get(meal_summary_last))
        .route("/:baby_id/meals/summary/range", get(meal_summary_range))
}

async fn get_meals(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    match date.get("date") {
        Some(d) => filter_meals_by_date_service(baby_id, d).await,
        None => get_meals_service(baby_id).await,
    }
}

async fn post_meal(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_meal): Json<NewMealDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    post_meal_service(new_meal, baby_id).await
}

async fn meal_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let string_date = parse_query_field(date, "date")?;
    meal_summary_service(baby_id, &string_date).await
}

async fn meal_summary_today(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    meal_summary_today_service(baby_id).await
}

async fn meal_summary_last(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let last_days: u64 = parse_query_field(date, "days")?.trim().parse().unwrap_or(7);
    meal_summary_last_days_service(baby_id, last_days).await
}

async fn meal_summary_range(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let from_date = parse_query_field(date.clone(), "from")?;
    let to_date = parse_query_field(date, "to")?;
    meal_summary_range_service(baby_id, &from_date, &to_date).await
}
