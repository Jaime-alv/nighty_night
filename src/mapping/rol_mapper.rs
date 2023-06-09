use crate::model::role_model::Rol;

pub fn translate_roles(roles: &[u8]) -> Vec<Rol>{
    roles.into_iter().map(|id| (*id).into()).collect()
}