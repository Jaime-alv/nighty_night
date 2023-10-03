use serde::Serialize;

use super::{
    admin_dto::{AdminBabyData, AdminUserData},
    baby_dto::BabyData,
    dream_dto::DreamData,
    meal_dto::MealData,
    role_dto::RoleData,
    session_dto::UserSessionData,
    user_dto::UserData,
    weight_dto::WeightData,
};

pub enum DataType {
    User,
    Baby,
    Dream,
    Meal,
    Weight,
    Role,
}

impl DataType {
    fn get(self) -> &'static str {
        match self {
            DataType::User => "user",
            DataType::Baby => "baby",
            DataType::Dream => "dream",
            DataType::Meal => "meal",
            DataType::Weight => "weight",
            DataType::Role => "role",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BasicDataStruct<T>
where
    T: Serialize,
{
    pub id: i32,
    r#type: &'static str,
    pub attributes: T,
}

impl<T> BasicDataStruct<T>
where
    T: Serialize,
{
    pub fn new(id: i32, r#type: DataType, attributes: T) -> Self {
        Self {
            id,
            r#type: r#type.get(),
            attributes,
        }
    }
}

pub type SessionDto = BasicDataStruct<UserSessionData>;
pub type UserDto = BasicDataStruct<UserData>;
pub type BabyDto = BasicDataStruct<BabyData>;
pub type DreamDto = BasicDataStruct<DreamData>;
pub type MealDto = BasicDataStruct<MealData>;
pub type WeightDto = BasicDataStruct<WeightData>;
pub type RoleDto = BasicDataStruct<RoleData>;
pub type AdminUserDto = BasicDataStruct<AdminUserData>;
pub type AdminBabyDto = BasicDataStruct<AdminBabyData>;
