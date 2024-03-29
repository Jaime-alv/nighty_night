use crate::connection::connection_psql::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::{
    users_babies::{self, baby_id},
    users_roles::{self, rol_id},
};

pub fn insert_rol_to_user(user: i32, rol: i16) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    let records: i64 = users_roles::table
        .filter(users_roles::rol_id.eq(rol))
        .filter(users_roles::user_id.eq(user))
        .count()
        .get_result(conn)?;
    match records {
        0 => diesel::insert_into(users_roles::table)
            .values((&rol_id.eq(rol), &users_roles::user_id.eq(user)))
            .execute(conn),
        _ => Ok(1),
    }
}

/// Look if there is an already association, if there is, return 1, else create a new association.
pub fn insert_baby_to_user(user: i32, baby: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    let records: i64 = users_babies::table
        .filter(users_babies::baby_id.eq(baby))
        .filter(users_babies::user_id.eq(user))
        .count()
        .get_result(conn)?;
    match records {
        0 => diesel::insert_into(users_babies::table)
            .values((&baby_id.eq(baby), &users_babies::user_id.eq(user)))
            .execute(conn),
        _ => Ok(1),
    }
}

pub fn delete_baby_association(baby: i32, user: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(
        users_babies::table
            .filter(users_babies::user_id.eq(user))
            .filter(users_babies::baby_id.eq(baby)),
    )
    .execute(conn)
}

pub fn delete_rol_to_user(user: i32, rol: i16) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(
        users_roles::table
            .filter(users_roles::user_id.eq(user))
            .filter(users_roles::rol_id.eq(rol)),
    )
    .execute(conn)
}
