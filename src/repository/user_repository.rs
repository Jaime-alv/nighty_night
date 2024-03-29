use crate::{
    data::query_dto::Pagination,
    model::{
        session_model::BabyInfo,
        user_model::{InsertableUser, User},
    },
    schema::{babies, users, users_babies, users_roles},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use super::paginator::Paginate;
use crate::connection::connection_psql::establish_connection;

///
/// Get all users from database.
pub fn select_all_users(pagination: Pagination) -> Result<(Vec<User>, i64), Error> {
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
pub fn select_user_by_username<T: Into<String>>(username: T) -> Result<User, Error> {
    let conn = &mut establish_connection();
    users::table
        .filter(users::username.eq(username.into()))
        .first(conn)
}

pub fn select_user_by_id(user_id: i32) -> Result<User, Error> {
    let conn = &mut establish_connection();
    users::table.find(user_id).first(conn)
}

pub fn insert_new_user<T: Into<InsertableUser>>(new_user: T, rol: i16) -> Result<User, Error> {
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

pub fn update_user(profile: User) -> Result<User, Error> {
    let conn = &mut establish_connection();
    diesel::update(users::table.find(profile.id()))
        .set((
            users::name.eq(profile.name()),
            users::surname.eq(profile.surname()),
            users::email.eq(profile.email()),
            users::updated_at.eq(profile.updated_at()),
        ))
        .get_result(conn)
}

pub fn update_active_for_user(
    user: i32,
    active: bool,
    time: NaiveDateTime,
) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(users::table.find(user))
        .set((users::active.eq(active), users::updated_at.eq(time)))
        .execute(conn)
}

pub fn delete_user(user: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(users::table.find(user)).execute(conn)
}

pub fn delete_all_users(older_than: NaiveDateTime) -> Result<usize, Error> {
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

/// Raw SQL:
///
/// ```sql
/// SELECT name, unique_id FROM babies WHERE id = babies_id
/// ```
pub fn select_babies_for_user_id(user: i32) -> Result<Vec<BabyInfo>, Error> {
    let conn = &mut establish_connection();
    let babies_id: Vec<i32> = users_babies::table
        .filter(users_babies::user_id.eq(user))
        .select(users_babies::baby_id)
        .load::<i32>(conn)?;
    let babies: Vec<BabyInfo> = babies::table
        .filter(babies::id.eq_any(babies_id))
        .select((babies::name, babies::unique_id))
        .load::<(String, Uuid)>(conn)?
        .into_iter()
        .map(|item| BabyInfo {
            name: item.0,
            unique_id: item.1,
        })
        .collect();
    Ok(babies)
}

pub fn select_user_from_username(username: &str) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    users::table
        .filter(users::username.eq(username))
        .execute(conn)
}
