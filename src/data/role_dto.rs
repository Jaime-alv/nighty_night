use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct RoleData {
    pub name: String,
    pub count: i64
}

#[derive(Deserialize)]
pub struct UpdateRole {
    pub username: String,
    pub role: String
}