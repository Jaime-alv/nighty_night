use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentUserDto {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub roles: Vec<u8>,
    pub active: bool,
    pub baby_id: Vec<i32>
}

impl CurrentUserDto {
    pub fn new(
        id: i64,
        anonymous: bool,
        username: String,
        roles: Vec<u8>,
        active: bool,
        baby_id: Vec<i32>
    ) -> Self {
        Self {
            id,
            anonymous,
            username,
            roles,
            active,
            baby_id
        }
    }
}
