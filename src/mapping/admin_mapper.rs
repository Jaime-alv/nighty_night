use crate::{
    data::admin_dto::{AdminBabyDto, AdminUserDto},
    model::{baby_model::Baby, user_model::User},
};

impl From<User> for AdminUserDto {
    fn from(user: User) -> Self {
        AdminUserDto {
            id: user.id(),
            username: user.username(),
            email: user.email(),
            active: user.active(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}

impl From<Baby> for AdminBabyDto {
    fn from(baby: Baby) -> Self {
        AdminBabyDto {
            id: baby.id(),
            name: baby.name(),
            belongs_to: baby.belongs_to(),
            added_on: baby.added_on(),
        }
    }
}
