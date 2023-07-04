use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MealDto {
    pub id: i32,
    pub date: String,
    pub time: String,
    pub quantity: i16,
    pub elapsed: String,
}

#[derive(Deserialize)]
pub struct NewMealDto {
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