use chrono::{Days, NaiveDate, NaiveDateTime};

use crate::{
    data::query_dto::Pagination,
    response::error::ApiError,
    utils::datetime::{convert_to_date_time, date_time_is_lower_than_other_date},
};

pub fn not_found() -> ApiError {
    ApiError::NotFound
}

pub fn uncover_date(date: Option<String>) -> Result<Option<NaiveDateTime>, ApiError> {
    match date {
        Some(d) => match convert_to_date_time(&d) {
            Ok(date) => Ok(Some(date)),
            Err(e) => Err(e.into()),
        },
        None => Ok(None),
    }
}

pub fn date_time_are_in_order(from: NaiveDateTime, to: NaiveDateTime) -> Result<(), ApiError> {
    if date_time_is_lower_than_other_date(from, to) {
        Ok(())
    } else {
        Err(ApiError::DatesUnordered)
    }
}

/// Checks if records' baby is the same as the one you want to perform the modification on.
pub fn record_belongs_to_baby(record_baby: i32, baby_id: i32) -> Result<(), ApiError> {
    if record_baby.ne(&baby_id) {
        Err(ApiError::Forbidden)
    } else {
        Ok(())
    }
}

pub fn records_is_not_empty<T>(records: Vec<T>) -> Result<Vec<T>, ApiError> {
    if records.is_empty() {
        Err(ApiError::NoRecord)
    } else {
        Ok(records)
    }
}

pub fn paginate_over_dates(
    pagination: Pagination,
    from: NaiveDate,
    to: NaiveDate,
) -> (NaiveDate, NaiveDate) {
    let days: u64 = ((pagination.page() - 1) * pagination.per_page())
        .try_into()
        .unwrap();
    let days_stop: u64 = ((pagination.page() * pagination.per_page()) - 1)
        .try_into()
        .unwrap();
    let start = from.checked_add_days(Days::new(days)).unwrap();
    let stop = from.checked_add_days(Days::new(days_stop)).unwrap();
    let start_date = if start.gt(&to) { to } else { start };
    let end_date = if stop.gt(&to) { to } else { stop };
    (start_date, end_date)
}

pub fn round_total_pages(from_date: NaiveDate, to_date: NaiveDate, per_page: i64) -> i64 {
    ((to_date - from_date).num_days() as f64 / per_page as f64).ceil() as i64
}

#[cfg(test)]
mod test_service {
    use super::*;

    #[test]
    fn test_pagination_dates() {
        let page_1 = Pagination::new(1, Some(10));
        let page_2 = Pagination::new(2, Some(10));
        let page_3 = Pagination::new(3, Some(10));
        let page_4 = Pagination::new(4, Some(10));
        let page_5 = Pagination::new(5, Some(10));
        let from = NaiveDate::from_ymd_opt(2023, 06, 01).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 07, 05).unwrap();
        assert_eq!(
            paginate_over_dates(page_1, from, to),
            (
                NaiveDate::from_ymd_opt(2023, 06, 01).unwrap(),
                NaiveDate::from_ymd_opt(2023, 06, 10).unwrap()
            )
        );
        assert_eq!(
            paginate_over_dates(page_2, from, to),
            (
                NaiveDate::from_ymd_opt(2023, 06, 11).unwrap(),
                NaiveDate::from_ymd_opt(2023, 06, 20).unwrap()
            )
        );
        assert_eq!(
            paginate_over_dates(page_3, from, to),
            (
                NaiveDate::from_ymd_opt(2023, 06, 21).unwrap(),
                NaiveDate::from_ymd_opt(2023, 06, 30).unwrap()
            )
        );
        assert_eq!(
            paginate_over_dates(page_4, from, to),
            (
                NaiveDate::from_ymd_opt(2023, 07, 01).unwrap(),
                NaiveDate::from_ymd_opt(2023, 07, 05).unwrap()
            )
        );
        assert_eq!(
            paginate_over_dates(page_5, from, to),
            (
                NaiveDate::from_ymd_opt(2023, 07, 05).unwrap(),
                NaiveDate::from_ymd_opt(2023, 07, 05).unwrap()
            )
        );
    }
}
