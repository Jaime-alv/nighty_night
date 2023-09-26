use crate::{
    data::user_dto::{FindUserDto, LoginDto, NewUserDto, UpdateUserDto},
    model::session_model::CurrentUser,
    service::{
        session_service::{
            get_current_user_service, login_required, login_session, logout_user_session,
        },
        user_service::{
            delete_active_user_service, delete_session_user_service, get_user_by_id_service,
            patch_user_service, post_find_user_service, post_new_user_service,
            post_session_user_service,
        },
    },
};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

pub(crate) fn route_user() -> Router {
    let routes = Router::new()
        .route("/register", post(post_new_user))
        .route("/user", post(post_find_user))
        .route(
            "/session",
            post(post_session_user)
                .delete(delete_session_user)
                .get(get_session_user),
        )
        .route(
            "/profile",
            get(get_user_by_id).patch(patch_user).delete(delete_user),
        );
    Router::new().nest("/auth", routes)
}

async fn post_new_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_user): Json<NewUserDto>,
) -> impl IntoResponse {
    match post_new_user_service(new_user).await {
        Ok((response, id)) => {
            login_session(auth, id).await?;
            Ok(response)
        }
        Err(error) => Err(error),
    }
}

async fn post_find_user(Json(data): Json<FindUserDto>) -> impl IntoResponse {
    post_find_user_service(data).await
}

async fn post_session_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(login): Json<LoginDto>,
) -> impl IntoResponse {
    match post_session_user_service(login).await {
        Ok((response, id)) => {
            login_session(auth, id).await?;
            Ok(response)
        }
        Err(error) => Err(error),
    }
}

async fn delete_session_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;
    match logout_user_session(auth, binding_id).await {
        Ok(_) => delete_session_user_service(),
        Err(error) => Err(error),
    }
}

async fn get_session_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    get_current_user_service(auth)
}

async fn get_user_by_id(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth)?;
    get_user_by_id_service(binding_id).await
}

async fn patch_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(profile): Json<UpdateUserDto>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth)?;
    patch_user_service(binding_id, profile).await
}

async fn delete_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;
    let delete_user_result = delete_active_user_service(binding_id, false).await?;
    match logout_user_session(auth, binding_id).await {
        Ok(_) => Ok(delete_user_result),
        Err(error) => Err(error),
    }
}
