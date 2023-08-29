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
        query_dto::{IdDto, Pagination},
        role_dto::UpdateRole,
    },
    model::session_model::CurrentUser,
    service::{
        admin_service::{display_roles_service, show_stats_service},
        association_service::{add_rol_to_user_service, delete_rol_to_user_service},
        baby_service::{find_baby_service, get_all_babies_service},
        session_service::current_user_is_admin,
        user_service::{
            alter_active_user_service, delete_old_users_service, delete_user_service,
            get_all_users_service,
        },
    },
};

pub(crate) fn route_admin() -> Router {
    let routes: Router = Router::new()
        .nest(
            "/baby",
            Router::new()
                .route("/", get(get_all_babies))
                .route("/:baby_id", get(find_baby_by_id)),
        )
        .route(
            "/user",
            get(get_all_users).delete(delete_user).patch(activate_user),
        )
        .route("/stats", get(show_records_stats))
        .route(
            "/roles",
            get(display_roles)
                .put(update_user_role)
                .delete(delete_user_role),
        );
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
    user_id: Option<Query<IdDto>>,
) -> impl IntoResponse {
    let binding: i32 = auth.id.try_into().unwrap();
    current_user_is_admin(auth)?;
    match user_id {
        Some(value) => delete_user_service(value.id(), binding).await,
        None => delete_old_users_service().await,
    }
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

async fn display_roles(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    display_roles_service().await
}

async fn update_user_role(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(user_role): Json<UpdateRole>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    add_rol_to_user_service(&user_role.username, user_role.role.into()).await
}

async fn delete_user_role(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(user_role): Json<UpdateRole>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    delete_rol_to_user_service(&user_role.username, user_role.role.into()).await
}
