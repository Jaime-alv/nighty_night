use chrono::{Days, NaiveDate};

use crate::{
    data::{dream_dto::DreamSummaryDto, query_dto::Pagination},
    model::{dream_model::Dream, summary_model::DreamSummary},
    repository::dream_repository::{find_dreams_summary, obtain_first_and_last_dream_date},
    response::{
        data_response::{PageInfo, PagedResponse},
        error::ApiError,
    },
    utils::datetime::{iter_between_two_dates, now, today},
};

use super::util_service::{paginate_over_dates, records_is_not_empty, round_total_pages};

pub async fn dream_summary_range_service(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamSummaryDto>>, ApiError> {
    let current = pagination.page();
    let total_pages = round_total_pages(from_date, to_date, pagination.per_page());
    let (start, stop) = paginate_over_dates(pagination, from_date, to_date);
    let summary = fetch_dream_summary_range(baby_id, start, stop).await?;
    let pager = PageInfo::new(current, total_pages);
    let response = PagedResponse::new(into_summary_dto(summary), pager);
    Ok(response)
}

/// Need to add plus one day to look for certain date.
async fn fetch_dream_summary_range(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<DreamSummary>, ApiError> {
    // dates_are_in_order(from_date, to_date)?;
    let plus_one = to_date.checked_add_days(Days::new(1)).unwrap();
    let mut summary_vec: Vec<DreamSummary> = Vec::new();
    let dreams = find_dreams_summary(baby_id, from_date, plus_one)?;
    let dates = iter_between_two_dates(from_date, plus_one);
    for day in dates {
        let partial_dreams = dreams
            .clone()
            .into_iter()
            .filter(|dream| dream.to_date().unwrap_or(now()).date().eq(&day))
            .collect::<Vec<Dream>>();
        if !partial_dreams.is_empty() {
            let summary = DreamSummary::new(day, partial_dreams);
            summary_vec.push(summary)
        }
    }
    records_is_not_empty(summary_vec)
}

pub async fn dream_summary_last_days_service(
    baby_id: i32,
    last_days: u32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamSummaryDto>>, ApiError> {
    let today = today();
    let from_date = today
        .checked_sub_days(Days::new(last_days.try_into().unwrap()))
        .unwrap();
    dream_summary_range_service(baby_id, from_date, today, pagination).await
}

fn into_summary_dto(summaries: Vec<DreamSummary>) -> Vec<DreamSummaryDto> {
    summaries.into_iter().map(|item| item.into()).collect()
}

pub async fn get_all_summary_records_paginated(
    baby_id: i32,
    pagination: Pagination,
) -> Result<PagedResponse<Vec<DreamSummaryDto>>, ApiError> {
    let current = pagination.page();
    let (raw_start, raw_stop) = obtain_first_and_last_dream_date(baby_id)?;
    let total_pages = round_total_pages(raw_start, raw_stop, pagination.per_page());
    let (start, stop) = paginate_over_dates(pagination, raw_start, raw_stop);
    let summary = fetch_dream_summary_range(baby_id, start, stop).await?;
    let pager = PageInfo::new(current, total_pages);
    let response = PagedResponse::new(into_summary_dto(summary), pager);
    Ok(response)
}
