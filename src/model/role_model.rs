use diesel::{Selectable, Identifiable, Queryable};

use crate::schema::roles;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    id: i32,
    name: String
}

impl From<Role> for Rol {
    fn from(value: Role) -> Self {
        match value.id {            
            0 => Rol::Admin,
            1 => Rol::User,
            2 => Rol::Anonymous,
            _ => Rol::Anonymous
        }
    }
}

impl From<u8> for Rol {
    fn from(value: u8) -> Self {
        match value {            
            0 => Rol::Admin,
            1 => Rol::User,
            2 => Rol::Anonymous,
            _ => Rol::Anonymous
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Rol {
    Anonymous,
    User,
    Admin
}
