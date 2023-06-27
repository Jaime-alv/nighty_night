use std::collections::HashSet;

use crate::{
    model::{
        user_model::{InsertableUser, User},
    },
    schema::{users, users_babies, users_roles},
};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

use super::connection_psql::{establish_async_connection};

///
/// Get all users from database.
pub async fn query_users() -> Result<Vec<User>, Error> {
    let conn = &mut establish_async_connection().await;
    users::table.load(conn).await
}

pub async fn load_user_by_username<T: Into<String>>(username: T) -> Result<User, Error> {
    let conn = &mut establish_async_connection().await;
    users::table
        .filter(users::username.eq(username.into()))
        .first(conn).await
}

pub async fn load_user_by_id(user_id: i32) -> Result<User, Error> {
    let conn = &mut establish_async_connection().await;
    users::table.find(user_id).first(conn).await
}

pub async fn create_user<T: Into<InsertableUser>>(new_user: T) -> Result<User, Error> {
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(users::table)
        .values(new_user.into())
        .returning(User::as_returning())
        .get_result(conn).await
    // .execute(conn)
}

pub async fn exists_username<T: Into<String>>(username: T) -> bool {
    match load_user_by_username(username.into()).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn find_roles_id(user_id: i32) -> HashSet<u8> {
    let mut roles: HashSet<u8> = HashSet::new();
    let conn = &mut establish_async_connection().await;
    users_roles::table
        .filter(users_roles::user_id.eq(user_id))
        .select(users_roles::rol_id)
        .load::<i16>(conn).await
        .unwrap()
        .iter()
        .for_each(|id| {
            roles.insert((*id).try_into().unwrap());
        });
    roles
}

pub async fn find_babies_id(user_id: i32) -> Vec<i32> {
    let conn = &mut establish_async_connection().await;
    users_babies::table
        .filter(users_babies::user_id.eq(user_id))
        .select(users_babies::baby_id)
        .load::<i32>(conn).await
        .unwrap()
}
