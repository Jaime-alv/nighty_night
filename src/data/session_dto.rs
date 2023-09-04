use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CurrentUserDto {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub roles: Vec<u8>,
    pub active: bool,
    pub baby_id: Vec<Uuid>
}

impl CurrentUserDto {
    pub fn new(
        id: i64,
        anonymous: bool,
        username: String,
        roles: Vec<u8>,
        active: bool,
        baby_id: Vec<Uuid>
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
