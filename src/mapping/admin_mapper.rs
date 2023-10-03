use crate::{
    data::{
        admin_dto::{AdminBabyData, AdminUserData},
        common_structure::{BasicDataStruct, DataType},
    },
    model::{baby_model::Baby, user_model::User},
};

impl From<User> for BasicDataStruct<AdminUserData> {
    fn from(user: User) -> Self {
        let attributes = AdminUserData {
            username: user.username(),
            email: user.email(),
            active: user.active(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
            roles: user.roles(),
        };
        BasicDataStruct::new(user.id(), DataType::User, attributes)
    }
}

impl From<Baby> for BasicDataStruct<AdminBabyData> {
    fn from(baby: Baby) -> Self {
        let attributes = AdminBabyData {
            name: baby.name(),
            belongs_to: baby.belongs_to(),
            added_on: baby.added_on(),
        };
        BasicDataStruct::new(baby.id(), DataType::Baby, attributes)
    }
}
