use diesel::prelude::*;
use diesel::result::Error;


use crate::schema::users_roles::{self, rol_id, user_id};

use super::connection_psql::establish_connection;

pub fn add_rol_to_user(user: i32, rol: i16) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    let target = users_roles::table;
    diesel::insert_into(target)
        .values((&rol_id.eq(rol), &user_id.eq(user)))
        .execute(conn)
}
