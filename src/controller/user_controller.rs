use crate::{
    data::user_dto::{FindUserDto, LoginDto, NewUserDto},
    model::session_model::CurrentUser,
    service::{
        response_service::forbidden,
        session_service::{is_admin, login_session},
        user_service::{
            create_user_service, find_user_service, get_all_users_service, login_service,
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
        .route("/all", get(get_all_users))
        .route("/user", post(find_user))
        .route("/login", post(login_user));
    Router::new().nest("/auth", routes)
}

async fn register_new_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_user): Json<NewUserDto>,
) -> impl IntoResponse {
    match create_user_service(new_user).await {
        Ok(user) => {
            login_session(auth, user.1).await?;
            Ok(Json(user.0))
        }
        Err(error) => Err(error),
    }
}

async fn get_all_users(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    if is_admin(auth).await {
        match get_all_users_service().await {
            Ok(list) => Ok(Json(list)),
            Err(error) => Err(error),
        }
    } else {
        Err(forbidden())
    }
}

async fn find_user(Json(data): Json<FindUserDto>) -> impl IntoResponse {
    match find_user_service(data).await {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err(error),
    }
}

async fn login_user(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(login): Json<LoginDto>,
) -> impl IntoResponse {
    match login_service(login).await {
        Ok(user) => {
            login_session(auth, user.1).await?;
            Ok(Json(user.0))
        }
        Err(error) => Err(error),
    }
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
