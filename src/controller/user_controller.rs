use crate::{
    data::user_dto::{FindUserDto, LoginDto, NewUserDto, UpdateUserDto},
    model::session_model::CurrentUser,
    service::{
        session_service::{login_required, login_session, logout_session},
        user_service::{
            alter_active_user_service, create_user_service, find_user_by_id_service,
            find_user_service, login_service, patch_user_service,
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
        .route("/", get(test_welcome))
        .route("/register", post(register_new_user))
        .route("/user", post(find_user))
        .route("/login", post(login_user))
        .route("/logout", get(logout_user))
        .route(
            "/profile",
            get(get_profile)
                .patch(update_profile)
                .delete(deactivate_user),
        );
    Router::new().nest("/auth", routes)
}

async fn register_new_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_user): Json<NewUserDto>,
) -> impl IntoResponse {
    match create_user_service(new_user).await {
        Ok((response, id)) => {
            login_session(auth, id).await?;
            Ok(response)
        }
        Err(error) => Err(error),
    }
}

async fn find_user(Json(data): Json<FindUserDto>) -> impl IntoResponse {
    find_user_service(data).await
}

async fn login_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(login): Json<LoginDto>,
) -> impl IntoResponse {
    match login_service(login).await {
        Ok((response, id)) => {
            login_session(auth, id).await?;
            Ok(response)
        }
        Err(error) => Err(error),
    }
}

async fn logout_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;
    logout_session(auth, binding_id).await
}

async fn test_welcome(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> String {
    auth.cache_clear_user(auth.id);
    format!(
        "Hello, {}.\n>>>This is a debug endpoint.<<<\nCredentials:\n{:#?}",
        auth.current_user.clone().unwrap().username(),
        auth.current_user.unwrap()
    )
}

async fn get_profile(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth)?;
    find_user_by_id_service(binding_id).await
}

async fn update_profile(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(profile): Json<UpdateUserDto>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth)?;
    patch_user_service(binding_id, profile).await
}

async fn deactivate_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth)?;
    alter_active_user_service(binding_id, false).await
}