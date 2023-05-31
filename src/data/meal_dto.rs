use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MealDto {
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    elapsed: Option<i16>
}

#[derive(Deserialize)]
pub struct NewMealDto {
    baby_id: i32,
    date: Option<NaiveDateTime>,
    quantity: Option<i16>,
    elapsed: Option<i16>
}