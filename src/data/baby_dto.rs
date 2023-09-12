use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InputBabyDto {
    pub name: Option<String>,
    pub birthdate: Option<String>,
}

#[derive(Serialize)]
pub struct BabyData {
    pub unique_id: Uuid,
    pub name: String,
    pub birthdate: String,    
}

pub struct UpdateBaby {
    pub name: String,
    pub birthdate: NaiveDate,
}
