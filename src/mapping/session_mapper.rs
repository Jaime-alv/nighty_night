use crate::{data::session_dto::CurrentUserDto, model::session_model::CurrentUser};

use super::rol_mapper::translate_roles;

impl From<CurrentUser> for CurrentUserDto {
    fn from(user: CurrentUser) -> Self {
        CurrentUserDto::new(
            user.id(),
            user.anonymous(),
            user.username(),
            user.roles_id(),
            user.active(),
            user.baby_id(),
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