use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::{
        meal_dto::NewMealDto,
        query_dto::{DateDto, DateRangeDto, LastDaysDto},
    },
    model::session_model::CurrentUser,
    service::{
        meal_service::{filter_meals_by_date_service, get_meals_service, post_meal_service},
        meal_summary_service::{
            get_all_meals_summaries_service, meal_summary_last_days_service,
            meal_summary_range_service, meal_summary_service, meal_summary_today_service,
        },
        session_service::authorize_and_has_baby,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new()
        .route("/:baby_id/meals", get(get_meals).post(post_meal))
        .route("/:baby_id/meals/summary", get(meal_summary))
        .route("/:baby_id/meals/summary/today", get(meal_summary_today))
        .route("/:baby_id/meals/summary/last", get(meal_summary_last))
        .route("/:baby_id/meals/summary/range", get(meal_summary_range))
        .route("/:baby_id/meals/summary/all", get(meal_summary_all))
}

async fn get_meals(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    date: Option<Query<DateDto>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    match date {
        Some(date) => filter_meals_by_date_service(baby_id, date.date()?).await,
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
    date: Query<DateDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    meal_summary_service(baby_id, date.date()?).await
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
    last_days: Query<LastDaysDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    meal_summary_last_days_service(baby_id, last_days.days()).await
}

async fn meal_summary_range(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    value: Query<DateRangeDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    meal_summary_range_service(baby_id, value.from()?, value.to()?).await
}

async fn meal_summary_all(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    get_all_meals_summaries_service(baby_id).await
}
