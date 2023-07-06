use chrono::{Duration, NaiveDate};

use crate::utils::datetime::{format_date, format_duration};

use super::{dream_model::Dream, meals_model::Meal};

#[derive(Clone)]
pub struct DreamSummary {
    date: NaiveDate,
    summary: Duration,
}

impl DreamSummary {
    pub fn new(date: NaiveDate, dreams: Vec<Dream>) -> Self {
        Self {
            date,
            summary: Self::sum_of_durations(dreams),
        }
    }

    fn sum_of_durations(dreams: Vec<Dream>) -> Duration {
        dreams
            .iter()
            .map(|d| d.elapsed())
            .reduce(|acc, e| acc.checked_add(&e).unwrap())
            .unwrap_or(Duration::minutes(0))
    }

    pub fn _date(&self) -> NaiveDate {
        self.date
    }

    pub fn formatted_date(&self) -> String {
        format_date(self.date)
    }

    pub fn summary(&self) -> Duration {
        self.summary
    }

    pub fn formatted_summary(&self) -> String {
        format_duration(self.summary.num_minutes())
    }
}

#[derive(Clone)]
pub struct MealSummary {
    date: NaiveDate,
    total_feedings: u8,
    nursing_time: Duration,
    formula: i16,
}

impl MealSummary {
    pub fn new(date: NaiveDate, meals: Vec<Meal>) -> Self {
        Self {
            date,
            total_feedings: Self::count_feedings(meals.len()),
            nursing_time: Self::sum_nursing_time(&meals),
            formula: Self::formula_feedings(&meals),
        }
    }

    fn count_feedings(intake: usize) -> u8 {
        intake.try_into().unwrap_or_default()
    }

    fn formula_feedings(meals: &Vec<Meal>) -> i16 {
        meals
            .into_iter()
            .map(|meal| meal.quantity())
            .reduce(|acc, feeds| acc + feeds)
            .unwrap_or_default()
    }

    fn sum_nursing_time(meals: &Vec<Meal>) -> Duration {
        meals
            .into_iter()
            .map(|meal| meal.elapsed())
            .reduce(|acc, e| acc.checked_add(&e).unwrap())
            .unwrap_or(Duration::minutes(0))
    }

    pub fn formatted_date(&self) -> String {
        format_date(self.date)
    }

    pub fn formatted_nursing_time(&self) -> String {
        format_duration(self.nursing_time.num_minutes())
    }

    pub fn total_feedings(&self) -> u8 {
        self.total_feedings
    }

    pub fn formula(&self) -> i16 {
        self.formula
    }
}
