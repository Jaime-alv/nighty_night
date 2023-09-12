use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
pub struct InputDreamDto {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Serialize)]
pub struct DreamSummaryDto {
    pub date: String,
    pub summary: String
}

pub struct UpdateDream {
    pub from_date: NaiveDateTime,
    pub to_date: Option<NaiveDateTime>
}

#[derive(Serialize)]
pub struct DreamData {
    pub from_date: String,
    pub from_time: String,
    pub to_date: String,
    pub to_time: String,
    pub elapsed: String
}