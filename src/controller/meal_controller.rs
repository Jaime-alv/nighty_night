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
        meal_dto::InputMealDto,
        query_dto::{AllRecords, DateDto, DateRangeDto, LastDaysDto, Pagination},
    },
    model::session_model::CurrentUser,
    service::{
        meal_service::{
            delete_meal_service, get_meal_id_service, get_meals_all_service,
            get_meals_by_last_days_service, get_meals_by_range_service, patch_meal_service,
            post_meal_service,
        },
        meal_summary_service::{
            get_meals_summary_all_service, get_meals_summary_last_days_service,
            get_meals_summary_range_service,
        },
        session_service::check_user_permissions,
    },
};

pub(super) fn route_meal() -> Router {
    Router::new().nest(
        "/meals",
        Router::new()
            .route("/", get(get_meals).post(post_meal))
            .route(
                "/:record",
                get(get_meal_id).patch(patch_meal).delete(delete_meal),
            )
            .route("/summary", get(get_meal_summary)),
    )
}

async fn get_meals(
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
        get_meals_all_service(baby_id, pagination).await
    } else if date.is_some() {
        let day = date.unwrap().date()?;
        get_meals_by_range_service(baby_id, day, day, pagination).await
    } else if range.is_some() {
        let dates = range.unwrap();
        get_meals_by_range_service(baby_id, dates.from()?, dates.to()?, pagination).await
    } else {
        let last = last_days.unwrap_or_default().days();
        get_meals_by_last_days_service(baby_id, last, pagination).await
    }
}

async fn post_meal(
    Path(baby_unique_id): Path<String>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(new_meal): Json<InputMealDto>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    post_meal_service(new_meal, baby_id).await
}

async fn patch_meal(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    Json(meal): Json<InputMealDto>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    patch_meal_service(meal, record, baby_id).await
}

async fn delete_meal(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    delete_meal_service(record, baby_id).await
}

/// Obtain summary records, if there are no parameters, it will try to get last 7 days.
async fn get_meal_summary(
    Path(baby_unique_id): Path<String>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    page: Option<Query<Pagination>>,
    all_records: Option<Query<AllRecords>>,
    date: Option<Query<DateDto>>,
    range: Option<Query<DateRangeDto>>,
    last_days: Option<Query<LastDaysDto>>,
) -> impl IntoResponse {
    let baby_id = check_user_permissions(auth, &baby_unique_id)?;
    let pagination = page.unwrap_or_default().0;
    if all_records.is_some() && all_records.unwrap().all() {
        get_meals_summary_all_service(baby_id, pagination).await
    } else if date.is_some() {
        get_meals_summary_range_service(
            baby_id,
            date.as_ref().unwrap().date()?,
            date.unwrap().date()?,
            pagination,
        )
        .await
    } else if range.is_some() {
        let range_date = range.unwrap();
        get_meals_summary_range_service(baby_id, range_date.from()?, range_date.to()?, pagination)
            .await
    } else {
        get_meals_summary_last_days_service(
            baby_id,
            last_days.unwrap_or_default().days(),
            pagination,
        )
        .await
    }
}

async fn get_meal_id(
    Path((baby_unique_id, record)): Path<(String, i32)>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    let baby_id: i32 = check_user_permissions(auth, &baby_unique_id)?;
    get_meal_id_service(record, baby_id).await
}
