use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewBabyDto {
    pub name: String,
    pub birthdate: String,
}

#[derive(Serialize)]
pub struct BabyDto {
    pub id: i32,
    pub name: String,
    pub birthdate: String
}
