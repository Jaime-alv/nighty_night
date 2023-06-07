use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::meal_dto::NewMealDto,
    model::session_model::CurrentUser,
    service::{
        meal_service::{get_meals_service, post_meal_service},
        response_service::forbidden,
        session_service::has_baby,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new().route("/:baby_id/meals", get(get_meals).post(post_meal))
}

async fn get_meals(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    if has_baby(auth, baby_id).await {
        match get_meals_service(baby_id).await {
            Ok(meals) => Ok(Json(meals)),
            Err(error) => Err(error),
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
