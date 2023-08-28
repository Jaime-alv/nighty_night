use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct AdminUserDto {
    pub id: i32,
    pub r#type: &'static str,
    pub attributes: AdminUserDtoAttributes,
}

#[derive(Serialize)]
pub struct AdminUserDtoAttributes {
    pub username: String,
    pub email: Option<String>,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AdminBabyDto {
    pub id: i32,
    pub r#type: &'static str,
    pub attributes: AdminBabyDtoAttributes,
}

#[derive(Serialize)]
pub struct AdminBabyDtoAttributes {
    pub name: String,
    pub belongs_to: i32,
    pub added_on: NaiveDateTime,
}
