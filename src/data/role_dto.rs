use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct RoleDto {
    pub id: i16,
    pub r#type: &'static str,
    pub attributes: RoleAttributes
}

#[derive(Serialize)]
pub struct RoleAttributes {
    pub name: String,
    pub count: i64
}

#[derive(Deserialize)]
pub struct UpdateRole {
    pub username: String,
    pub role: String
}