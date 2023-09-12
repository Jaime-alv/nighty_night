use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputWeightDto {
    pub date: Option<String>,
    pub value: Option<f32>,
}

pub struct UpdateWeight {
    pub date: NaiveDate,
    pub value: f32,
}

#[derive(Serialize)]
pub struct WeightData {
    pub date: String,
    pub value: f32,
}
