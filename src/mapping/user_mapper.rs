use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        user_dto::{NewUserDto, UserData},
    },
    model::user_model::{InsertableUser, User},
    security::security::hash_password,
    utils::datetime::now,
};

impl From<User> for BasicDataStruct<UserData> {
    fn from(user: User) -> Self {
        let attributes = UserData {
            username: user.username(),
            email: user.email(),
            name: user.name(),
            surname: user.surname(),
        };
        BasicDataStruct::new(user.id(), DataType::User, attributes)
    }
}

impl From<&User> for BasicDataStruct<UserData> {
    fn from(user: &User) -> BasicDataStruct<UserData> {
        let attributes = UserData {
            username: user.username(),
            email: user.email(),
            name: user.name(),
            surname: user.surname(),
        };
        BasicDataStruct::new(user.id(), DataType::User, attributes)
    }
}

impl From<NewUserDto> for InsertableUser {
    fn from(new_user: NewUserDto) -> Self {
        let hash = hash_password(new_user.password);
        InsertableUser::new(
            new_user.username,
            hash,
            new_user.email,
            new_user.name,
            new_user.surname,
            now(),
        )
    }
}
