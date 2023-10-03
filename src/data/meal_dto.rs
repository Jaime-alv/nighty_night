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
    pub formula: i16,
}

#[derive(Serialize)]
pub struct MealData {
    pub date: String,
    pub start_time: String,
    pub quantity: i16,
    pub elapsed: String,
}
