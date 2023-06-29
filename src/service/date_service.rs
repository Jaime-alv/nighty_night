use chrono::NaiveDateTime;

use crate::{error::error::ApiError, utils::datetime::to_date_time};

pub fn uncover_date(date: Option<String>) -> Result<Option<NaiveDateTime>, ApiError> {
    match date {
        Some(d) => match to_date_time(&d) {
            Ok(date) => Ok(Some(date)),
            Err(e) => Err(e.into()),
        },
        None => Ok(None),
    }
}
