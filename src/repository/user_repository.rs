use crate::{
    model::{associations_model::UserBaby, baby_model::Baby, user_model::{User, InsertableUser}},
    schema::{babies, users, users_babies},
};
use diesel::prelude::*;
use diesel::result::Error;

use super::connection::establish_connection;

///
/// Get all users from database.
pub fn query_users() -> Result<Vec<User>, Error> {
    let conn = &mut establish_connection();
    users::table.load(conn)
}

pub fn load_user<T: Into<String>>(username: T) -> Result<User, Error> {
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
    let user_id = UserBaby::belonging_to(user).select(users_babies::baby_id);
    babies::table
        .filter(babies::id.eq_any(user_id))
        .load::<Baby>(conn)
        .expect("could not load babies from user.")
}

pub fn exists<T: Into<String>>(username: T) -> bool {
    match load_user(username.into()) {
        Ok(_) => true,
        Err(_) => false,
    }
}
