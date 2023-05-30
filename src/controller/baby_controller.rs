use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::baby_dto::NewBabyDto,
    error::error::ApiError,
    model::session_model::CurrentUser,
    service::{
        baby_service::{find_baby_service, get_all_babies_service, ingest_new_baby},
        user_service::{find_user_by_username_service},
    },
};

pub(crate) fn route_baby() -> Router {
    let routes: Router = Router::new()
        .route("/new", post(register_baby))
        // .route("/new", post(register_baby_with_username))
        .route("/:baby_id", get(find_baby_by_id))
        .route("/all", get(get_all_babies));
    Router::new().nest("/baby", routes)
}

async fn register_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_baby): Json<NewBabyDto>,
) -> impl IntoResponse {
    if auth.is_authenticated() {
        let id: i32 = auth.id.try_into().unwrap();
        match ingest_new_baby(new_baby, id).await {
            Ok(baby) => Ok(Json(baby)),
            Err(error) => Err(error),
        }
    } else {
        Err(ApiError::Forbidden)
    }
}

async fn find_baby_by_id(Path(baby_id): Path<i32>) -> impl IntoResponse {
    match find_baby_service(baby_id).await {
        Ok(baby) => Ok(Json(baby)),
        Err(error) => Err(error),
    }
}

async fn register_baby_with_username(
    Query(user): Query<HashMap<String, String>>,
    Json(new_baby): Json<NewBabyDto>,
) -> impl IntoResponse {
    let user = user.get("username").expect("Expected username.");
    let current_user = match find_user_by_username_service(user).await {
        Ok(u) => u,
        Err(error) => return Err(error),
    };
    match ingest_new_baby(new_baby, current_user.id()).await {
        Ok(baby) => Ok(Json(baby)),
        Err(error) => Err(error),
    }
}

async fn get_all_babies() -> impl IntoResponse {
    match get_all_babies_service().await {
        Ok(list) => Ok(Json(list)),
        Err(error) => Err(error),
    }
}
