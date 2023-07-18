use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    data::baby_dto::NewBabyDto,
    model::session_model::CurrentUser,
    service::{
        baby_service::{find_baby_service, get_all_babies_service, ingest_new_baby},
        session_service::{current_user_is_admin, login_required, update_user_session},
    },
};

use super::{
    dream_controller::route_dream, meal_controller::route_meal, weight_controller::route_weight,
};

pub(crate) fn route_baby() -> Router {
    let routes: Router = Router::new()
        .route("/new", post(register_baby))
        .route("/:baby_id", get(find_baby_by_id))
        .route("/all", get(get_all_babies))
        .merge(route_meal())
        .merge(route_dream())
        .merge(route_weight());
    Router::new().nest("/baby", routes)
}

async fn register_baby(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_baby): Json<NewBabyDto>,
) -> impl IntoResponse {
    let id: i32 = auth.id.try_into().unwrap();
    login_required(auth.clone())?;    
    match ingest_new_baby(new_baby, id).await {
        Ok(baby) => {
            update_user_session(&auth.current_user.unwrap()).await?;
            Ok(baby)
        }
        Err(error) => Err(error),
    }
}

async fn find_baby_by_id(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    find_baby_service(baby_id).await
}

async fn get_all_babies(
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    current_user_is_admin(auth)?;
    get_all_babies_service().await
}
