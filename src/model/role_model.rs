use diesel::{Identifiable, Queryable, Selectable};

use crate::schema::roles;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    id: i16,
    name: String,
}

impl From<Role> for Rol {
    fn from(value: Role) -> Self {
        match value.id {
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

#[derive(PartialEq, Clone, Debug)]
pub enum Rol {
    Anonymous,
    User,
    Admin,
}
