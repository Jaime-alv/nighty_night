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
    data::{
        meal_dto::InputMealDto,
        query_dto::{AllRecords, DateDto, DateRangeDto, IdDto, LastDaysDto, Pagination},
    },
    model::session_model::CurrentUser,
    service::{
        meal_service::{
            delete_meal_service, filter_meals_by_last_days, filter_meals_by_range,
            get_all_meals_paginated_service, patch_meal_service, post_meal_service,
        },
        meal_summary_service::{
            get_all_summary_records_paginated, meal_summary_last_days_service,
            meal_summary_range_service,
        },
        session_service::authorize_and_has_baby_unique_id,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new().nest(
        "/meals",
        Router::new()
            .route(
                "/",
                get(get_meals)
                    .post(post_meal)
                    .patch(patch_meal)
                    .delete(delete_meal),
            )
            .route("/summary", get(meal_summary)),
    )
}

async fn get_meals(
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
        get_all_meals_paginated_service(baby_id, pagination).await
    } else if date.is_some() {
        let day = date.unwrap().date()?;
        filter_meals_by_range(baby_id, day, day, pagination).await
    } else if range.is_some() {
        let dates = range.unwrap();
        filter_meals_by_range(baby_id, dates.from()?, dates.to()?, pagination).await
    } else {
        let last = last_days.unwrap_or_default().days();
        filter_meals_by_last_days(baby_id, last, pagination).await
    }
}

async fn post_meal(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_meal): Json<InputMealDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    post_meal_service(new_meal, baby_id).await
}

async fn patch_meal(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
    Json(meal): Json<InputMealDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    patch_meal_service(meal, record_id.id(), baby_id).await
}

async fn delete_meal(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    delete_meal_service(record_id.id(), baby_id).await
}

/// Obtain summary records, if there are no parameters, it will try to get last 7 days.
async fn meal_summary(
    Path(baby_unique_id): Path<Uuid>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    page: Option<Query<Pagination>>,
    all_records: Option<Query<AllRecords>>,
    date: Option<Query<DateDto>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    let baby_id = authorize_and_has_baby_unique_id(auth, baby_unique_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_all_summary_records_paginated(baby_id, pagination).await
    } else if date.is_some() {
        meal_summary_range_service(
            baby_id,
            date.as_ref().unwrap().date()?,
            date.unwrap().date()?,
            pagination,
        )
        .await
    } else if range.is_some() {
        let range_date = range.unwrap();
        meal_summary_range_service(baby_id, range_date.from()?, range_date.to()?, pagination).await
    } else {
        meal_summary_last_days_service(baby_id, last_days.unwrap_or_default().days(), pagination)
            .await
    }
}
