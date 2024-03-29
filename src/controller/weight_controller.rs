use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;

use crate::{
    configuration::constant::GlobalCte,
    data::{
        query_dto::{AllRecords, DateDto, DateRangeDto, LastDaysDto, Pagination},
        weight_dto::InputWeightDto,
    },
    model::session_model::CurrentUser,
    service::{
        session_service::check_user_permissions,
        weight_service::{
            delete_weight_service, get_weight_id_service, get_weight_range_service,
            get_weights_all_service, get_weights_by_last_days, patch_weight_service,
            post_weight_service,
        },
    },
};

pub(super) fn route_weight() -> Router {
    Router::new().nest(
        "/weights",
        Router::new()
            .route(
                "/",
                get(get_weights)
                    .post(post_weight)
                    .patch(patch_weight)
                    .delete(delete_weight),
            )
            .route(
                "/:record",
                get(get_weight_id).patch(patch_weight).delete(delete_weight),
            ),
    )
}

async fn get_weights(
    Path(baby_unique_id): Path<String>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    all_records: Option<Query<AllRecords>>,
    date: Option<Query<DateDto>>,
    page: Option<Query<Pagination>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_weights_all_service(baby_id, pagination).await
    } else if date.is_some() {
        let day = date.unwrap().date()?;
        get_weight_range_service(baby_id, day, day, pagination).await
    } else if range.is_some() {
        let dates = range.unwrap();
        get_weight_range_service(baby_id, dates.from()?, dates.to()?, pagination).await
    } else {
        let last = last_days
            .unwrap_or(axum::extract::Query(LastDaysDto::new(
                GlobalCte::WeightLastDaysDefault.get(),
            )))
            .days();
        get_weights_by_last_days(baby_id, last, pagination).await
    }
}

async fn post_weight(
    Path(baby_unique_id): Path<String>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    post_weight_service(new_measure, baby_id).await
}

async fn patch_weight(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    patch_weight_service(measure, record, baby_id).await
}

async fn delete_weight(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    delete_weight_service(record, baby_id).await
}

async fn get_weight_id(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    get_weight_id_service(record, baby_id).await
}
