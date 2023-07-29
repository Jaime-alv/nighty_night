use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::query_dto::{IdDto, Pagination},
    model::session_model::CurrentUser,
    service::{
        admin_service::show_stats_service,
        baby_service::{find_baby_service, get_all_babies_service},
        session_service::current_user_is_admin,
        user_service::{alter_active_user_service, delete_user_service, get_all_users_service},
    },
};

pub(crate) fn route_admin() -> Router {
    let routes: Router = Router::new()
        .route("/baby", get(get_all_babies))
        .route(
            "/user",
            get(get_all_users).delete(delete_user).patch(activate_user),
        )
        .route("/baby/:baby_id", get(find_baby_by_id))
        .route("/stats", get(show_records_stats));
    Router::new().nest("/admin", routes)
}

async fn get_all_babies(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    page: Option<Query<Pagination>>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    let pagination = page.unwrap_or_default().0;
    get_all_babies_service(pagination).await
}

async fn get_all_users(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    page: Option<Query<Pagination>>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    let pagination = page.unwrap_or_default().0;
    get_all_users_service(pagination).await
}

async fn delete_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: Query<IdDto>,
) -> impl IntoResponse {
    let binding: i32 = auth.id.try_into().unwrap();
    current_user_is_admin(auth)?;
    delete_user_service(user_id.id(), binding).await
}

async fn activate_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: Query<IdDto>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    alter_active_user_service(user_id.id(), true).await
}

async fn find_baby_by_id(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    find_baby_service(baby_id).await
}

async fn show_records_stats(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    show_stats_service().await
}
