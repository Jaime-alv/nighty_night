use chrono::{NaiveDate, NaiveDateTime};
use hyper::StatusCode;

use crate::{
    error::error::ApiError,
    utils::{
        datetime::{date_is_lower_than_other_date, to_date_time},
        response::Response,
    },
};

pub fn ok(msg: &str) -> Response {
    Response::new(StatusCode::OK, msg)
}

pub fn not_found() -> ApiError {
    ApiError::NotFound
}

pub fn uncover_date(date: Option<String>) -> Result<Option<NaiveDateTime>, ApiError> {
    match date {
        Some(d) => match to_date_time(&d) {
            Ok(date) => Ok(Some(date)),
            Err(e) => Err(e.into()),
        },
        None => Ok(None),
    }
}

pub fn check_days_out_of_bounds(days: u64) -> Result<(), ApiError> {
    if days.ge(&0) && days.lt(&61) {
        Ok(())
    } else {
        Err(ApiError::OutOfBounds(0, 60))
    }
}

pub fn dates_are_in_order(from: NaiveDate, to: NaiveDate) -> Result<(), ApiError> {
    if date_is_lower_than_other_date(from, to) {
        Ok(())
    } else {
        Err(ApiError::DatesUnordered)
    }
}
