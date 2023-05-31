use std::collections::HashSet;

use crate::{
    model::{
        associations_model::UserBaby,
        baby_model::Baby,
        user_model::{InsertableUser, User},
    },
    schema::{babies, users, users_babies, users_roles},
};
use diesel::prelude::*;
use diesel::result::Error;

use super::connection_psql::establish_connection;

///
/// Get all users from database.
pub fn query_users() -> Result<Vec<User>, Error> {
    let conn = &mut establish_connection();
    users::table.load(conn)
}

pub fn load_user_by_username<T: Into<String>>(username: T) -> Result<User, Error> {
    let conn = &mut establish_connection();
    users::table
        .filter(users::username.eq(username.into()))
        .first(conn)
}

pub fn load_user_by_id(user_id: i32) -> Result<User, Error> {
    let conn = &mut establish_connection();
    users::table.find(user_id).first(conn)
}

pub fn create_user<T: Into<InsertableUser>>(new_user: T) -> Result<User, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(users::table)
        .values(new_user.into())
        .returning(User::as_returning())
        .get_result(conn)
    // .execute(conn)
}

// #[axum_macros::debug_handler]
pub fn find_related_babies(user: &User) -> Vec<Baby> {
    let conn = &mut establish_connection();
    let baby_id = UserBaby::belonging_to(user).select(users_babies::baby_id);
    babies::table
        .filter(babies::id.eq_any(baby_id))
        .load::<Baby>(conn)
        .expect("could not load babies from user.")
}

pub fn exists_username<T: Into<String>>(username: T) -> bool {
    match load_user_by_username(username.into()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn find_roles_id(user_id: i32) -> HashSet<u8> {
    let mut roles: HashSet<u8> = HashSet::new();
    let conn = &mut establish_connection();
    users_roles::table
        .filter(users_roles::user_id.eq(user_id))
        .select(users_roles::rol_id)
        .load::<i16>(conn)
        .unwrap()
        .iter()
        .for_each(|id| {
            roles.insert((*id).try_into().unwrap());
        });
    roles
}

pub fn find_babies_id(user_id: i32) -> Vec<i32> {
    let conn = &mut establish_connection();
    users_babies::table
        .filter(users_babies::user_id.eq(user_id))
        .select(users_babies::baby_id)
        .load::<i32>(conn)
        .unwrap()
}
