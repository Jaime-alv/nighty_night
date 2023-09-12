use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        role_dto::RoleData,
    },
    model::role_model::{Rol, Role},
    repository::admin_repository::GroupedRole,
};

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

impl From<GroupedRole> for BasicDataStruct<RoleData> {
    fn from(value: GroupedRole) -> Self {
        let attributes = RoleData {
            name: value.name,
            count: value.count,
        };
        BasicDataStruct::new(value.id.into(), DataType::Role, attributes)
    }
}

impl From<Rol> for String {
    fn from(value: Rol) -> Self {
        match value {
            Rol::Anonymous => "anonymous".to_string(),
            Rol::User => "user".to_string(),
            Rol::Admin => "admin".to_string(),
        }
    }
}

impl From<String> for Rol {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "anonymous" => Rol::Anonymous,
            "admin" => Rol::Admin,
            _ => Rol::User,
        }
    }
}
