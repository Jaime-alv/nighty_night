use chrono::{NaiveDateTime, NaiveDate};

use crate::{utils::datetime::{to_date_time, to_date}, error::error::ApiError};

pub fn uncover_date(date: Option<String>) -> Result<Option<NaiveDateTime>, ApiError> {
    match date {
        Some(d) => match to_date_time(&d) {
            Ok(date) => Ok(Some(date)),
            Err(e) => Err(ApiError::DateFormat(e)),
        }
        None => Ok(None),
    }
}

pub fn parse_date(string_date: &str) -> Result<NaiveDate, ApiError> {
    match to_date(&string_date) {
        Ok(d) => Ok(d),
        Err(error) => Err(ApiError::DateFormat(error)),
    }
}

pub fn parse_timestamp(date_time: &str) -> Result<NaiveDateTime, ApiError> {
    match to_date_time(date_time) {
        Ok(d) => Ok(d),
        Err(e) => Err(ApiError::DateFormat(e)),
    }
}