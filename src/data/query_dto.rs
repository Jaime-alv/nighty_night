use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    error::error::ApiError,
    utils::datetime::{convert_to_date, format_date, today},
};

#[derive(Deserialize)]
pub struct DateDto {
    date: String,
}

impl DateDto {
    pub fn date(&self) -> Result<NaiveDate, ApiError> {
        parse_date(&self.date)
    }
}

impl Default for DateDto {
    fn default() -> Self {
        Self {
            date: format_date(today()),
        }
    }
}

#[derive(Deserialize)]
pub struct LastDaysDto {
    days: Option<u64>,
}

impl LastDaysDto {
    pub fn days(&self) -> u64 {
        self.days.unwrap_or(7)
    }
}

#[derive(Deserialize)]
pub struct DateRangeDto {
    from: String,
    to: Option<String>,
}

impl DateRangeDto {
    pub fn from(&self) -> Result<NaiveDate, ApiError> {
        parse_date(&self.from)
    }

    pub fn to(&self) -> Result<NaiveDate, ApiError> {
        match &self.to {
            Some(value) => parse_date(&value),
            None => Ok(today()),
        }
    }
}

fn parse_date(date: &str) -> Result<NaiveDate, ApiError> {
    match convert_to_date(date) {
        Ok(date) => Ok(date),
        Err(error) => Err(error.into()),
    }
}
