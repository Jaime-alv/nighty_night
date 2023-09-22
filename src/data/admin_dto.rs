use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct AdminUserData {
    pub username: String,
    pub email: Option<String>,
    pub active: bool,
    pub roles: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AdminBabyData {
    pub name: String,
    pub belongs_to: i32,
    pub added_on: NaiveDateTime,
}
