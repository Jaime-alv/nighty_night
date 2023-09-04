use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_session::SessionRedisPool;
use axum_session_auth::AuthSession;
use uuid::Uuid;

use crate::{
    configuration::constant::GlobalCte,
    data::{
        query_dto::{AllRecords, DateDto, DateRangeDto, IdDto, LastDaysDto, Pagination},
        weight_dto::InputWeightDto,
    },
    model::session_model::CurrentUser,
    service::{
        session_service::authorize_and_has_baby_unique_id,
        weight_service::{
            delete_weight_service, filter_weights_by_last_days, get_weight_range_service,
            get_weights_service, patch_weight_service, post_weight_service,
        },
    },
};

pub(super) fn route_weight() -> Router {
    Router::new().route(
        "/weights",
        get(get_weights)
            .post(post_weight)
            .patch(patch_weight)
            .delete(delete_weight),
    )
}

async fn get_weights(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    all_records: Option<Query<AllRecords>>,
    date: Option<Query<DateDto>>,
    page: Option<Query<Pagination>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_weights_service(baby_id, pagination).await
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
        filter_weights_by_last_days(baby_id, last, pagination).await
    }
}

async fn post_weight(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    post_weight_service(new_measure, baby_id).await
}

async fn patch_weight(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record: Query<IdDto>,
    Json(measure): Json<InputWeightDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    patch_weight_service(measure, record.id(), baby_id).await
}

async fn delete_weight(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    delete_weight_service(record_id.id(), baby_id).await
}
