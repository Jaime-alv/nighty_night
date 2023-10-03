use serde::{Deserialize, Serialize};
use crate::model::session_model::BabyInfo;

/// This struct belongs to redis repository.
/// 
/// If you need to use a dto for UI use
/// [UserSessionData]
#[derive(Serialize, Deserialize)]
pub struct CurrentUserDto {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
    pub roles: Vec<i16>,
    pub active: bool,
    pub baby_id: Vec<BabyInfo>,
}

impl CurrentUserDto {
    pub fn new(
        id: i64,
        anonymous: bool,
        username: String,
        roles: Vec<i16>,
        active: bool,
        baby_id: Vec<BabyInfo>,
    ) -> Self {
        Self {
            id,
            anonymous,
            username,
            roles,
            active,
            baby_id,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserSessionData {
    pub username: String,
    pub baby_info: Vec<BabyInfo>,
}
