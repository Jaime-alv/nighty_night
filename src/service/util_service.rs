use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    configuration::constant::GlobalCte,
    error::error::ApiError,
    utils::datetime::{
        convert_to_date_time, date_is_lower_than_other_date, date_time_is_lower_than_other_date,
    },
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

pub fn check_days_out_of_bounds(days: u32) -> Result<(), ApiError> {
    if days.ge(&0) && days.lt(&GlobalCte::DaysOutOfBoundsCte.get()) {
        Ok(())
    } else {
        Err(ApiError::OutOfBounds(
            0,
            GlobalCte::DaysOutOfBoundsCte.get(),
        ))
    }
}

pub fn dates_are_in_order(from: NaiveDate, to: NaiveDate) -> Result<(), ApiError> {
    if date_is_lower_than_other_date(from, to) {
        Ok(())
    } else {
        Err(ApiError::DatesUnordered)
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

pub fn check_dates_are_in_bound(from: NaiveDate, to: NaiveDate) -> Result<(), ApiError> {
    let days: u32 = (to - from).num_days().try_into()?;
    check_days_out_of_bounds(days)
}


pub fn records_is_not_empty<T>(records: Vec<T>) -> Result<Vec<T>, ApiError> {
    if records.is_empty() {
        Err(ApiError::NoRecord)
    } else {
        Ok(records)
    }
}