use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputBabyDto {
    pub name: Option<String>,
    pub birthdate: Option<String>,
}

#[derive(Serialize)]
pub struct BabyDto {
    pub id: i32,
    pub r#type: &'static str,
    pub attributes: BabyAttributes,
}

pub struct UpdateBaby {
    pub name: String,
    pub birthdate: NaiveDate,
}

#[derive(Serialize)]
pub struct BabyAttributes {
    pub name: String,
    pub birthdate: String,
}
