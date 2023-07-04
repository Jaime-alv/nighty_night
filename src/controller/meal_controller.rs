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
            filter_meals_by_date_service, get_meals_service, meal_summary_service,
            post_meal_service,
        },
        session_service::authorize_and_has_baby,
        util_service::parse_query_field,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new()
        .route("/:baby_id/meals", get(get_meals).post(post_meal))
        .route("/:baby_id/meals/summary", get(meal_summary))
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

async fn meal_summary_range(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let from_date = parse_query_field(date, "from")?;
    meal_summary_service(baby_id, &from_date).await
}
