use crate::model::role_model::{Rol, Role};

pub fn translate_roles(roles: &[u8]) -> Vec<Rol> {
    roles.into_iter().map(|id| (*id).into()).collect()
}

impl From<Role> for Rol {
    fn from(value: Role) -> Self {
        match value.id() {
            0 => Rol::Admin,
            1 => Rol::User,
            2 => Rol::Anonymous,
            _ => Rol::Anonymous,
        }
    }
}

impl From<u8> for Rol {
    fn from(value: u8) -> Self {
        match value {
            0 => Rol::Admin,
            1 => Rol::User,
            2 => Rol::Anonymous,
            _ => Rol::Anonymous,
        }
    }
}

impl From<Rol> for i16 {
    fn from(rol: Rol) -> Self {
        match rol {
            Rol::Anonymous => 2,
            Rol::User => 1,
            Rol::Admin => 0,
        }
    }
}

impl From<Rol> for u8 {
    fn from(rol: Rol) -> Self {
        match rol {
            Rol::Anonymous => 2,
            Rol::User => 1,
            Rol::Admin => 0,
        }
    }
}
