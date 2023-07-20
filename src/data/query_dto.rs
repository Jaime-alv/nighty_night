use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    error::error::ApiError,
    utils::datetime::{convert_to_date, format_date, today},
};

#[derive(Deserialize)]
pub struct IdDto {
    entry: i32,
}

impl IdDto {
    pub fn id(&self) -> i32 {
        self.entry
    }
}

#[derive(Deserialize)]
pub struct DateDto {
    date: String,
}

impl DateDto {
    pub fn date(&self) -> Result<NaiveDate, ApiError> {
        match self.date.as_str() {
            "today" => Ok(today()),
            _ => parse_date(&self.date),
        }
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
    last_days: u64,
}

impl Default for LastDaysDto {
    fn default() -> Self {
        Self { last_days: 7 }
    }
}

impl LastDaysDto {
    pub fn days(&self) -> u64 {
        self.last_days
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

#[derive(Deserialize)]
pub struct AllRecords {
    all: bool
}

impl AllRecords {
    pub fn all(&self) -> bool {
        self.all
    }
}

fn parse_date(date: &str) -> Result<NaiveDate, ApiError> {
    match convert_to_date(date) {
        Ok(date) => Ok(date),
        Err(error) => Err(error.into()),
    }
}

#[derive(Deserialize)]
pub struct Username {
    value: Option<String>,
}

impl Username {
    pub fn value(&self) -> Option<String> {
        self.value.to_owned()
    }
}
