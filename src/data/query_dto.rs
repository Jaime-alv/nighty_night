use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    configuration::constant::GlobalCte,
    response::error::ApiError,
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
    last_days: u32,
}

impl Default for LastDaysDto {
    fn default() -> Self {
        Self {
            last_days: GlobalCte::LastDaysCte.get(),
        }
    }
}

impl LastDaysDto {
    pub fn new(days: u32) -> Self {
        Self { last_days: days }
    }
    pub fn days(&self) -> u32 {
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
        // If from() is a future day of to(), it will return from's date.
        // If there is no value, it will return today's date.
        let value = match &self.to {
            Some(value) => {
                let to_date = parse_date(&value);
                Self::compare_dates(&self, to_date)
            }
            None => Ok(today()),
        };
        value
    }

    fn compare_dates(&self, to_date: Result<NaiveDate, ApiError>) -> Result<NaiveDate, ApiError> {
        let date = match to_date {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        if self.from()?.le(&date) {
            to_date
        } else {
            self.from()
        }
    }
}

#[derive(Deserialize)]
pub struct AllRecords {
    all: bool,
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
    username: Option<String>,
}

impl Username {
    pub fn username(&self) -> Result<String, ApiError> {
        match self.username.to_owned() {
            Some(value) => Ok(value),
            None => Err(ApiError::NoUser),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: i32,
    per_page: Option<i32>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: Some(GlobalCte::RecordsPerPage.get().try_into().unwrap()),
        }
    }
}

impl Pagination {
    pub fn new(page: i32, per_page: Option<i32>) -> Self {
        Self { page, per_page }
    }

    pub fn page(&self) -> i64 {
        self.page.into()
    }

    pub fn per_page(&self) -> i64 {
        let threshold: i64 = GlobalCte::MaxPaginationThreshold.get().try_into().unwrap();
        match self.per_page {
            Some(quantity) => {
                let bind: i64 = quantity.into();
                if bind.gt(&threshold) {
                    threshold
                } else {
                    bind
                }
            }
            None => GlobalCte::RecordsPerPage.get().try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod test_query {
    use super::*;
    #[test]
    fn test_date_order() {
        let correct_date_order = DateRangeDto {
            from: "2023-06-01".to_string(),
            to: Some("2023-06-03".to_string()),
        };
        assert_eq!(
            correct_date_order.to().unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 3).unwrap()
        );
        let incorrect_date_order = DateRangeDto {
            from: "2023-06-03".to_string(),
            to: Some("2023-06-01".to_string()),
        };
        assert_eq!(
            incorrect_date_order.to().unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 3).unwrap()
        );
    }

    #[test]
    fn test_input_date_range() {
        let today = today();
        let invalid_date = DateRangeDto {
            from: "2023-06-bb".to_string(),
            to: None,
        };
        assert!(invalid_date.from().is_err());
        assert_eq!(invalid_date.to().unwrap(), today);
        let invalid_date_to = DateRangeDto {
            from: "2023-06-01".to_string(),
            to: Some("2023-06-bb".to_string()),
        };
        assert!(invalid_date_to.to().is_err());
    }

    #[test]
    fn test_date() {
        let date = DateDto {
            date: "today".to_string(),
        };
        assert_eq!(date.date().unwrap(), today());
    }
}
