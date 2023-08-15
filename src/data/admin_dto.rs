use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct AdminUserDto {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AdminBabyDto {
    pub id: i32,
    pub name: String,
    pub belongs_to: i32,
    pub added_on: NaiveDateTime
}