use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        session_dto::{CurrentUserDto, SessionUserDto, UserSessionData},
    },
    model::session_model::CurrentUser,
};

use super::rol_mapper::translate_roles;

impl From<CurrentUser> for CurrentUserDto {
    fn from(user: CurrentUser) -> Self {
        CurrentUserDto::new(
            user.id(),
            user.anonymous(),
            user.username(),
            user.roles_id(),
            user.active(),
            user.baby_info(),
        )
    }
}

impl From<CurrentUserDto> for CurrentUser {
    fn from(user: CurrentUserDto) -> Self {
        CurrentUser::new(
            user.id,
            user.anonymous,
            user.username,
            translate_roles(&user.roles),
            user.active,
            user.baby_id,
        )
    }
}

impl From<CurrentUser> for SessionUserDto {
    fn from(user: CurrentUser) -> Self {
        SessionUserDto {
            id: user.id(),
            username: user.username(),
            roles: user.roles().into_iter().map(|rol| rol.into()).collect(),
            baby_info: user.baby_info(),
        }
    }
}

impl From<CurrentUser> for BasicDataStruct<UserSessionData> {
    fn from(value: CurrentUser) -> Self {
        let attributes = UserSessionData {
            username: value.username(),
            roles: value.roles().into_iter().map(|rol| rol.into()).collect(),
            baby_info: value.baby_info(),
        };
        BasicDataStruct::new(value.id().try_into().unwrap(), DataType::User, attributes)
    }
}
