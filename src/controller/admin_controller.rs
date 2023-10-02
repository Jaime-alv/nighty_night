use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::{
        query_dto::{IdDto, Pagination},
        role_dto::UpdateRole,
    },
    model::session_model::CurrentUser,
    service::{
        admin_service::{get_roles_service, get_stats_of_tables_service},
        association_service::{add_rol_to_user_service, delete_rol_to_user_service},
        baby_service::{get_all_babies_service, get_baby_by_id_service},
        session_service::current_user_is_admin,
        user_service::{
            delete_active_user_service, delete_old_users_service, delete_user_with_time_constrain_service,
            get_all_users_service, get_user_id_from_username,
        }, role_service::get_role_by_name_service,
    },
};

pub(crate) fn route_admin() -> Router {
    let routes: Router = Router::new()
        .nest(
            "/baby",
            Router::new()
                .route("/", get(get_all_babies))
                .route("/baby_id", get(get_baby_by_id)),
        )
        .route(
            "/user",
            get(get_all_users)
                .delete(delete_user)
                .patch(patch_activate_user),
        )
        .route("/stats", get(get_stats_of_tables))
        .route(
            "/roles",
            get(get_roles).put(put_user_role).delete(delete_user_role),
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
        Some(value) => delete_user_with_time_constrain_service(value.id(), binding).await,
        None => delete_old_users_service().await,
    }
}

async fn patch_activate_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    user_id: Query<IdDto>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    delete_active_user_service(user_id.id(), true).await
}

async fn get_baby_by_id(
    baby_id: Query<IdDto>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    get_baby_by_id_service(baby_id.id()).await
}

async fn get_stats_of_tables(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    get_stats_of_tables_service().await
}

async fn get_roles(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    get_roles_service().await
}

async fn put_user_role(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(user_role): Json<UpdateRole>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    let rol = get_role_by_name_service(&user_role.role).await?;
    let user = get_user_id_from_username(&user_role.username).await?;
    add_rol_to_user_service(user, rol).await
}

async fn delete_user_role(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(user_role): Json<UpdateRole>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    let rol = get_role_by_name_service(&user_role.role).await?;
    let user = get_user_id_from_username(&user_role.username).await?;
    delete_rol_to_user_service(user, rol).await
}
