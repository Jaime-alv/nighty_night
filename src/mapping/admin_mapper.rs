use crate::{
    data::admin_dto::{AdminBabyDto, AdminBabyDtoAttributes, AdminUserDto, AdminUserDtoAttributes},
    model::{baby_model::Baby, user_model::User},
};

use super::data_type::DataType;

impl From<User> for AdminUserDto {
    fn from(user: User) -> Self {
        let attr = AdminUserDtoAttributes {
            username: user.username(),
            email: user.email(),
            active: user.active(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        };
        AdminUserDto {
            id: user.id(),
            r#type: DataType::User.get(),
            attributes: attr,
        }
    }
}

impl From<Baby> for AdminBabyDto {
    fn from(baby: Baby) -> Self {
        let attr = AdminBabyDtoAttributes {
            name: baby.name(),
            belongs_to: baby.belongs_to(),
            added_on: baby.added_on(),
        };
        AdminBabyDto {
            id: baby.id(),
            r#type: DataType::Baby.get(),
            attributes: attr,
        }
    }
}
