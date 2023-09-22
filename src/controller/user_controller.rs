use crate::{
    data::user_dto::{FindUserDto, LoginDto, NewUserDto, UpdateUserDto},
    model::session_model::CurrentUser,
    service::{
        session_service::{login_required, login_session, get_logout_user_service},
        user_service::{
            delete_active_user_service, post_new_user_service, get_user_by_id_service,
            post_find_user_service, login_service, patch_user_service,
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
        .route("/", get(get_test_welcome))
        .route("/register", post(post_new_user))
        .route("/user", post(post_find_user))
        .route("/login", post(post_login_user))
        .route("/logout", get(get_logout_user))
        .route(
            "/profile",
            get(get_user_by_id)
                .patch(patch_user)
                .delete(delete_user),
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

async fn post_login_user(
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

async fn get_logout_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let binding_id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;
    get_logout_user_service(auth, binding_id).await
}

async fn get_test_welcome(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> String {
    auth.cache_clear_user(auth.id);
    format!(
        "Hello, {}.\n>>>This is a debug endpoint.<<<\nCredentials:\n{:#?}",
        auth.current_user.clone().unwrap().username(),
        auth.current_user.unwrap()
    )
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
    login_required(auth)?;
    delete_active_user_service(binding_id, false).await
}
