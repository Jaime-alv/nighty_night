use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
pub struct NewDreamDto {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Serialize)]
pub struct DreamDto {
    pub from_date: String,
    pub from_time: String,
    pub to_date: String,
    pub to_time: String,
}
