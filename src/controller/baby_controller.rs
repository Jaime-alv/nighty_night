use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;
use uuid::Uuid;

use crate::{
    data::{
        baby_dto::InputBabyDto,
        query_dto::{Pagination, Username},
    },
    model::session_model::CurrentUser,
    service::{
        association_service::add_baby_to_user_service,
        baby_service::{
            delete_baby_service, find_baby_service, ingest_new_baby, load_babies_for_current_user,
            patch_baby_service, transfer_baby_service,
        },
        session_service::{authorize_and_has_baby_unique_id, login_required, update_user_session},
    },
};

use super::{
    dream_controller::route_dream, meal_controller::route_meal, weight_controller::route_weight,
};

pub(crate) fn route_baby() -> Router {
    let routes: Router = Router::new()
        .route("/", get(get_babies_for_user).post(register_baby))
        .nest(
            "/:baby_id",
            Router::new()
                .route(
                    "/",
                    get(find_baby_by_id).patch(patch_baby).delete(delete_baby),
                )
                .route("/share", post(share_ownership))
                .route("/transfer", patch(transfer_owner))
                .merge(route_meal())
                .merge(route_dream())
                .merge(route_weight()),
        );
    Router::new().nest("/baby", routes)
}

async fn register_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_baby): Json<InputBabyDto>,
) -> impl IntoResponse {
    let id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;
    match ingest_new_baby(new_baby, id).await {
        Ok(baby) => {
            update_user_session(auth).await?;
            Ok(baby)
        }
        Err(error) => Err(error),
    }
}

async fn find_baby_by_id(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    find_baby_service(baby_id).await
}

async fn patch_baby(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(update): Json<InputBabyDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    patch_baby_service(baby_id, update).await
}

async fn delete_baby(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let user_binding: i32 = auth.id.try_into().unwrap();
    let baby_id = authorize_and_has_baby_unique_id(auth.clone(), baby_unique_id)?;
    let message = delete_baby_service(baby_id, user_binding).await;
    if message.is_ok() {
        update_user_session(auth).await?;
    }
    message
}

async fn get_babies_for_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    page: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let id: i64 = auth.id;
    let pagination = page.unwrap_or_default().0;
    login_required(auth)?;
    load_babies_for_current_user(id, pagination).await
}

async fn share_ownership(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user: Query<Username>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    let username = user.username()?;
    add_baby_to_user_service(baby_id, &username).await
}

async fn transfer_owner(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user: Query<Username>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    let username = user.username()?;
    transfer_baby_service(baby_id, &username).await
}
