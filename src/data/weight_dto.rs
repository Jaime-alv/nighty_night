use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputWeightDto {
    pub date: Option<String>,
    pub value: Option<f32>,
}

#[derive(Serialize, Debug)]
pub struct WeightData {
    pub date: String,
    pub value: f32,
}
