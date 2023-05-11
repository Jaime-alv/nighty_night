use crate::{
    data::user_dto::{FindUserDto, LoginDto, NewUserDto},
    service::user_service::{
        create_user_service, find_user_service, get_all_users_service, login_service,
    },
};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub(crate) fn route_user() -> Router {
    let routes = Router::new()
        .route("/register", post(register_new_user))
        .route("/all", get(get_all_users))
        .route("/user", post(find_user))
        .route("/login", post(login_user));
    Router::new().nest("/auth", routes)
}

async fn register_new_user(Json(new_user): Json<NewUserDto>) -> impl IntoResponse {
    match create_user_service(new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

async fn get_all_users() -> impl IntoResponse {
    Json(get_all_users_service().await)
}

async fn find_user(Json(data): Json<FindUserDto>) -> impl IntoResponse {
    match find_user_service(data).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

async fn login_user(Json(login): Json<LoginDto>) -> impl IntoResponse {
    match login_service(login).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}
