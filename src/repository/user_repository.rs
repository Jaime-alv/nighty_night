use std::collections::HashSet;

use crate::{
    data::{query_dto::Pagination, user_dto::UpdateUser},
    model::user_model::{InsertableUser, User},
    schema::{users, users_babies, users_roles},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;

use super::{connection_psql::establish_connection, paginator::Paginate};

///
/// Get all users from database.
pub fn query_users(pagination: Pagination) -> Result<(Vec<User>, i64), Error> {
    let conn = &mut establish_connection();
    users::table
        .select(users::all_columns)
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

/// Raw SQL:
///
/// ```sql
/// SELECT * FROM users WHERE username = ${username};
/// ```
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

pub fn create_user<T: Into<InsertableUser>>(new_user: T, rol: i16) -> Result<User, Error> {
    let conn = &mut establish_connection();
    // Create user entry in db.
    let user: Result<User, Error> = diesel::insert_into(users::table)
        .values(new_user.into())
        .returning(User::as_returning())
        .get_result(conn);
    let binding = match user {
        Ok(ref value) => value.id(),
        Err(e) => return Err(e),
    };
    // Associate user and rol.
    diesel::insert_into(users_roles::table)
        .values((
            &users_roles::rol_id.eq(rol),
            &users_roles::user_id.eq(binding),
        ))
        .execute(conn)?;
    user
}

pub fn _exists_username<T: Into<String>>(username: T) -> bool {
    match load_user_by_username(username.into()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn find_roles_id(user_id: i32) -> Result<HashSet<u8>, Error> {
    let mut roles: HashSet<u8> = HashSet::new();
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

pub fn find_babies_id(user_id: i32) -> Result<HashSet<i32>, Error> {
    let mut babies: HashSet<i32> = HashSet::new();
    let conn = &mut establish_connection();
    users_babies::table
        .filter(users_babies::user_id.eq(user_id))
        .select(users_babies::baby_id)
        .load::<i32>(conn)?
        .into_iter()
        .for_each(|id| {
            babies.insert(id);
        });
    Ok(babies)
}

pub fn patch_user_record(user_id: i32, profile: UpdateUser) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(users::table.find(user_id))
        .set((
            users::name.eq(profile.name),
            users::surname.eq(profile.surname),
            users::email.eq(profile.email),
            users::updated_at.eq(profile.update_at),
        ))
        .execute(conn)
}

pub fn alter_active_status_for_user(
    user: i32,
    active: bool,
    time: NaiveDateTime,
) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(users::table.find(user))
        .set((users::active.eq(active), users::updated_at.eq(time)))
        .execute(conn)
}

pub fn delete_user_from_db(user: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(users::table.find(user)).execute(conn)
}

pub fn delete_users_from_db_in_batch(older_than: NaiveDateTime) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(
        users::table
            .filter(users::active.eq(false))
            .filter(users::updated_at.le(older_than)),
    )
    .execute(conn)
}

/// Raw SQL:
/// 
/// ```sql
/// SELECT id FROM users WHERE username = ${username};
/// ```
pub fn select_id_from_username(username: &str) -> Result<i32, Error> {
    let conn = &mut establish_connection();
    users::table
        .filter(users::username.eq(username))
        .select(users::id)
        .first(conn)
}
