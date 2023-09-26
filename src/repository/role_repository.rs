use std::collections::HashSet;

use diesel::prelude::*;
use diesel::result::Error;

use crate::{
    connection::connection_psql::establish_connection,
    model::role_model::Role,
    schema::{roles, users_roles},
};

pub fn select_roles_names_from_user(user_id: i32) -> Result<Vec<String>, Error> {
    let conn = &mut establish_connection();
    let roles_id: Vec<i16> = users_roles::table
        .filter(users_roles::user_id.eq(user_id))
        .select(users_roles::rol_id)
        .load::<i16>(conn)?;
    let role_alias: Result<Vec<String>, Error> = roles::table
        .filter(roles::id.eq_any(roles_id))
        .select(roles::name)
        .load::<String>(conn);
    role_alias
}

pub fn select_roles_id_from_user(user_id: i32) -> Result<HashSet<i16>, Error> {
    let mut roles: HashSet<i16> = HashSet::new();
    let conn = &mut establish_connection();
    users_roles::table
        .filter(users_roles::user_id.eq(user_id))
        .select(users_roles::rol_id)
        .load::<i16>(conn)?
        .iter()
        .for_each(|id| {
            roles.insert((*id).try_into().unwrap());
        });
    Ok(roles)
}

pub fn select_role_from_role_name(rol_name: &str) -> Result<Role, Error> {
    let conn = &mut establish_connection();
    roles::table
        .filter(roles::name.eq(rol_name))
        .get_result(conn)
}
