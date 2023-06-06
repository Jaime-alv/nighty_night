use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MealDto {
    pub date: String,
    pub time: String,
    pub quantity: i16,
    pub elapsed: i16,
}

#[derive(Deserialize)]
pub struct NewMealDto {
    pub date: Option<String>,
    pub quantity: Option<i16>,
    pub elapsed: Option<i16>,
}
