use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        role_dto::RoleData,
    },
    model::role_model::Rol,
    repository::admin_repository::GroupedRole,
};

pub fn translate_roles(roles: &[i16]) -> Vec<Rol> {
    roles.into_iter().map(|id| (*id).into()).collect()
}

impl From<i16> for Rol {
    fn from(value: i16) -> Self {
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

impl From<GroupedRole> for BasicDataStruct<RoleData> {
    fn from(value: GroupedRole) -> Self {
        let attributes = RoleData {
            name: value.name,
            count: value.count,
        };
        BasicDataStruct::new(value.id.into(), DataType::Role, attributes)
    }
}
