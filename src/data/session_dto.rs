use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentUserDto {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub roles: Vec<u8>,
    pub active: bool,
}

impl CurrentUserDto {
    pub fn new(id: i64, anonymous: bool, username: String, roles: Vec<u8>, active: bool) -> Self {
        Self {
            id,
            anonymous,
            username,
            roles,
            active
        }
    }
}
