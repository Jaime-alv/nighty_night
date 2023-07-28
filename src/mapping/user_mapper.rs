use crate::{
    data::user_dto::{AdminUserDto, NewUserDto, UserDto},
    model::user_model::{InsertableUser, User},
    security::security::hash_password,
    utils::datetime::now,
};

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto::new(user.username(), user.email(), user.name(), user.surname())
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> UserDto {
        UserDto::new(user.username(), user.email(), user.name(), user.surname())
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
