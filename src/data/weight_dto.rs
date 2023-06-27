use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct WeightDto {
    pub id: i32,
    pub date: String,
    pub value: f32
}

#[derive(Deserialize)]
pub struct NewWeightDto {
    pub date: String,
    pub value: f32
}