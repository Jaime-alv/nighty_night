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
        response_service::{empty_query, forbidden},
        session_service::has_baby,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new()
        .route("/:baby_id/meals", get(get_meals).post(post_meal))
        .route("/:baby_id/meals/summary", get(meal_summary))
}

async fn get_meals(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match date.get("date") {
            Some(d) => match filter_meals_by_date_service(baby_id, d).await {
                Ok(meals) => Ok(Json(meals)),
                Err(error) => Err(error),
            },
            None => match get_meals_service(baby_id).await {
                Ok(meals) => Ok(Json(meals)),
                Err(error) => Err(error),
            },
        }
    } else {
        Err(forbidden().await)
    }
}

async fn post_meal(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_meal): Json<NewMealDto>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        post_meal_service(new_meal, baby_id).await
    } else {
        Err(forbidden().await)
    }
}

async fn meal_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Query(date): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match date.get("date") {
            Some(string_date) => match meal_summary_service(baby_id, string_date).await {
                Ok(meals) => Ok(Json(meals)),
                Err(error) => Err(error),
            },
            None => Err(empty_query()),
        }
    } else {
        Err(forbidden().await)
    }
}
