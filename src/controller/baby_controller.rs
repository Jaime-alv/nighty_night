use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    data::baby_dto::NewBabyDto,
    service::{
        baby_service::{find_baby_service, get_all_babies_service, ingest_new_baby},
        user_service::{find_user_by_id_service, find_user_by_username_service},
    },
};

pub(crate) fn route_baby() -> Router {
    let routes: Router = Router::new()
        .route("/new/:user_id", post(register_baby))
        .route("/new", post(register_baby_with_username))
        .route("/:baby_id", get(find_baby_by_id))
        .route("/all", get(get_all_babies));
    Router::new().nest("/baby", routes)
}

async fn register_baby(
    Path(user_id): Path<i32>,
    Json(new_baby): Json<NewBabyDto>,
) -> impl IntoResponse {
    let current_user = match find_user_by_id_service(user_id).await {
        Ok(u) => u,
        Err(error) => return Err(error),
    };
    match ingest_new_baby(new_baby, current_user).await {
        Ok(baby) => Ok(Json(baby)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(e)
        }
    }
}

async fn find_baby_by_id(Path(baby_id): Path<i32>) -> impl IntoResponse {
    match find_baby_service(baby_id).await {
        Ok(baby) => Ok(Json(baby)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(e)
        }
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
    match ingest_new_baby(new_baby, current_user).await {
        Ok(baby) => Ok(Json(baby)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(e)
        }
    }
}

async fn get_all_babies() -> impl IntoResponse {
    match get_all_babies_service().await {
        Ok(list) => Ok(Json(list)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(e)
        }
    }
}
