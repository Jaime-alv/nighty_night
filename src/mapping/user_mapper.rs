use crate::{
    data::user_dto::{NewUserDto, UserDto},
    model::user_model::{InsertableUser, User},
    security::security::hash_password,
};

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto::new(
            user.username(),
            user.email(),
            user.name(),
            user.surname(),
            user.find_related_babies_names(),
        )
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> UserDto {
        UserDto::new(
            user.username(),
            user.email(),
            user.name(),
            user.surname(),
            user.find_related_babies_names(),
        )
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
        )
    }
}

pub fn users_to_users_dto(users: Vec<User>) -> Vec<UserDto> {
    users.into_iter().map(|u| UserDto::from(u)).collect()
}
