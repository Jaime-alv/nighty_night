use crate::connection::connection_psql::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::{
    data::query_dto::Pagination,
    model::baby_model::{Baby, InsertableBaby},
    schema::{babies, users_babies},
};

use super::paginator::Paginate;

pub fn insert_new_baby<T>(new_baby: T, user: i32) -> Result<Baby, Error>
where
    T: Into<InsertableBaby>,
{
    let conn = &mut establish_connection();
    // Create baby entry in db.
    let baby = diesel::insert_into(babies::table)
        .values(new_baby.into())
        .returning(Baby::as_returning())
        .get_result(conn);
    let binding = match baby {
        Ok(ref value) => value.id(),
        Err(e) => return Err(e),
    };
    // Associate baby and user.
    diesel::insert_into(users_babies::table)
        .values((
            &users_babies::baby_id.eq(binding),
            &users_babies::user_id.eq(user),
        ))
        .execute(conn)?;
    baby
}

pub fn select_baby_by_id(id: i32) -> Result<Baby, Error> {
    let conn = &mut establish_connection();
    babies::table.find(id).first(conn)
}

pub fn select_babies(pagination: Pagination) -> Result<(Vec<Baby>, i64), Error> {
    let conn = &mut establish_connection();
    babies::table
        .select(babies::all_columns)
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn update_baby(update: Baby) -> Result<Baby, Error> {
    let conn = &mut establish_connection();
    diesel::update(babies::table.find(update.id()))
        .set((
            babies::name.eq(update.name()),
            babies::birthdate.eq(update.birthdate()),
        ))
        .get_result(conn)
}

pub fn delete_baby_from_db(baby: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(babies::table.find(baby)).execute(conn)
}

pub fn update_baby_belongs_to(baby: i32, new_user: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(babies::table.find(baby))
        .set(babies::belongs_to.eq(new_user))
        .execute(conn)
}

pub fn select_baby_from_unique_id(unique_id: Uuid) -> Result<i32, Error> {
    let conn = &mut establish_connection();
    babies::table
        .filter(babies::unique_id.eq(unique_id))
        .select(babies::id)
        .first(conn)
}

pub fn select_babies_from_user_id(
    user_id: i32,
    pagination: Pagination,
) -> Result<(Vec<Baby>, i64), Error> {
    let conn = &mut establish_connection();
    let babies_id: Vec<i32> = users_babies::table
        .filter(users_babies::user_id.eq(user_id))
        .select(users_babies::baby_id)
        .load::<i32>(conn)?;
    babies::table
        .filter(babies::id.eq_any(babies_id))
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}
