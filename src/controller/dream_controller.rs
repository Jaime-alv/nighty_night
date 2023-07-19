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
        query_dto::{DateDto, DateRangeDto, IdDto, LastDaysDto},
    },
    model::session_model::CurrentUser,
    service::{
        dream_service::{
            delete_dream_service, filter_dreams_by_date_service, get_all_dreams_from_baby_service,
            patch_dream_service, post_dream_service,
        },
        dream_summary_service::{
            dream_summary_last_days_service, dream_summary_range_service, dream_summary_service,
            dream_summary_today_service, get_all_dream_summaries_service,
        },
        session_service::authorize_and_has_baby,
    },
};

pub(super) fn route_dream() -> Router {
    Router::new()
        .route(
            "/:baby_id/dreams",
            get(get_dreams)
                .post(post_dream)
                .patch(patch_dream)
                .delete(delete_dream),
        )
        .route("/:baby_id/dreams/summary", get(dream_summary))
        .route("/:baby_id/dreams/summary/today", get(dream_summary_today))
        .route("/:baby_id/dreams/summary/last", get(dream_summary_last))
        .route("/:baby_id/dreams/summary/range", get(dream_summary_range))
        .route("/:baby_id/dreams/summary/all", get(dream_summary_all))
}

async fn get_dreams(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    date: Option<Query<DateDto>>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    match date {
        Some(date) => filter_dreams_by_date_service(baby_id, date.date()?).await,
        None => get_all_dreams_from_baby_service(baby_id).await,
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

async fn dream_summary(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    date: Query<DateDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    let day = date.date()?;
    dream_summary_service(baby_id, day).await
}

async fn dream_summary_today(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    dream_summary_today_service(baby_id).await
}

async fn dream_summary_last(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    last_days: Query<LastDaysDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    dream_summary_last_days_service(baby_id, last_days.days()).await
}

async fn dream_summary_range(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    value: Query<DateRangeDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    dream_summary_range_service(baby_id, value.from()?, value.to()?).await
}

async fn dream_summary_all(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    get_all_dream_summaries_service(baby_id).await
}

async fn delete_dream(
    Path(baby_id): Path<i32>,
    auth: AuthSession<CurrentUser, i64, SessionRedisPool, redis::Client>,
    record_id: Query<IdDto>,
) -> impl IntoResponse {
    authorize_and_has_baby(auth, baby_id)?;
    delete_dream_service(record_id.id(), baby_id).await
}
