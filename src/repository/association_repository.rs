use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::{
    users_babies::{self, baby_id},
    users_roles::{self, rol_id},
};

use super::connection_psql::establish_connection;

pub fn add_rol_to_user(user: i32, rol: i16) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(users_roles::table)
        .values((&rol_id.eq(rol), &users_roles::user_id.eq(user)))
        .execute(conn)
}

pub fn add_baby_to_user(user: i32, baby: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(users_babies::table)
        .values((&baby_id.eq(baby), &users_babies::user_id.eq(user)))
        .execute(conn)
}
