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
        dream_dto::InputDreamDto,
        query_dto::{AllRecords, DateDto, DateRangeDto, IdDto, LastDaysDto, Pagination},
    },
    model::session_model::CurrentUser,
    service::{
        dream_service::{
            delete_dream_service, filter_dreams_by_last_days, get_dreams_by_range_date,
            get_dreams_paginated_service, patch_dream_service, post_dream_service,
        },
        dream_summary_service::{
            dream_summary_last_days_service, dream_summary_range_service,
            get_all_summary_records_paginated,
        },
        session_service::authorize_and_has_baby,
    },
};

pub(super) fn route_dream() -> Router {
    Router::new().nest(
        "/dreams",
        Router::new()
            .route(
                "/",
                get(get_dreams)
                    .post(post_dream)
                    .patch(patch_dream)
                    .delete(delete_dream),
            )
            .route("/summary", get(dream_summary)),
    )
}

async fn get_dreams(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    all_records: Option<Query<AllRecords>>,
    date: Option<Query<DateDto>>,
    page: Option<Query<Pagination>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_dreams_paginated_service(baby_id, pagination).await
    } else if date.is_some() {
        let day = date.unwrap().date()?;
        get_dreams_by_range_date(baby_id, day, day, pagination).await
    } else if range.is_some() {
        let dates = range.unwrap();
        get_dreams_by_range_date(baby_id, dates.from()?, dates.to()?, pagination).await
    } else {
        let last = last_days.unwrap_or_default().days();
        filter_dreams_by_last_days(baby_id, last, pagination).await
    }
}

async fn post_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_dream): Json<InputDreamDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    post_dream_service(new_dream, baby_id).await
}

async fn patch_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
    Json(dream): Json<InputDreamDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    patch_dream_service(dream, record_id.id(), baby_id).await
}

async fn delete_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    delete_dream_service(record_id.id(), baby_id).await
}

/// Obtain summary records, if there are no parameters, it will try to get last 7 days.
async fn dream_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    all_records: Option<Query<AllRecords>>,
    page: Option<Query<Pagination>>,
    date: Option<Query<DateDto>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_all_summary_records_paginated(baby_id, pagination).await
    } else if date.is_some() {
        dream_summary_range_service(
            baby_id,
            date.as_ref().unwrap().date()?,
            date.unwrap().date()?,
            pagination,
        )
        .await
    } else if range.is_some() {
        let range_date = range.unwrap();
        dream_summary_range_service(baby_id, range_date.from()?, range_date.to()?, pagination).await
    } else {
        dream_summary_last_days_service(baby_id, last_days.unwrap_or_default().days(), pagination)
            .await
    }
}
