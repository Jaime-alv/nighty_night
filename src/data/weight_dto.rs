use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct WeightDto {
    pub id: i32,
    pub r#type: &'static str,
    pub attributes: WeightAttributes,
}

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
pub struct WeightAttributes {
    pub date: String,
    pub value: f32,
}
