use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct InputMealDto {
    pub date: Option<String>,
    pub quantity: Option<i16>,
    pub to_time: Option<String>,
}


#[derive(Serialize)]
pub struct MealSummaryDto {
    pub date: String,
    pub total_feedings: u8,
    pub nursing_time: String,
    pub formula: i16
}

pub struct UpdateMeal {
    pub date: NaiveDateTime,
    pub quantity: Option<i16>,
    pub to_time: Option<NaiveDateTime>
}

#[derive(Serialize)]
pub struct MealData {
    pub date: String,
    pub time: String,
    pub quantity: i16,
    pub elapsed: String,
}