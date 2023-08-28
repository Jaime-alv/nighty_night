use crate::{
    data::user_dto::{NewUserDto, UserAttributes, UserDto},
    model::user_model::{InsertableUser, User},
    security::security::hash_password,
    utils::datetime::now,
};

use super::data_type::DataType;

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        let attr = UserAttributes {
            username: user.username(),
            email: user.email(),
            name: user.name(),
            surname: user.surname(),
        };
        UserDto {
            id: user.id(),
            r#type: DataType::User.get(),
            attributes: attr,
        }
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> UserDto {
        let attr = UserAttributes {
            username: user.username(),
            email: user.email(),
            name: user.name(),
            surname: user.surname(),
        };
        UserDto {
            id: user.id(),
            r#type: DataType::User.get(),
            attributes: attr,
        }
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
