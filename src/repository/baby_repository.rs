use diesel::prelude::*;
use diesel::result::Error;

use crate::{
    data::{baby_dto::UpdateBaby, query_dto::Pagination},
    model::baby_model::{Baby, InsertableBaby},
    schema::{babies, users_babies},
};

use super::{connection_psql::establish_connection, paginator::Paginate};

pub fn ingest_new_baby_in_db<T>(new_baby: T, user: i32) -> Result<Baby, Error>
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

pub fn load_baby_by_id(id: i32) -> Result<Baby, Error> {
    let conn = &mut establish_connection();
    babies::table.find(id).first(conn)
}

pub fn query_babies(pagination: Pagination) -> Result<(Vec<Baby>, u32), Error> {
    let conn = &mut establish_connection();
    babies::table
        .select(babies::all_columns)
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn patch_baby_record(baby: i32, update: UpdateBaby) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(babies::table.find(baby))
        .set((
            babies::name.eq(update.name),
            babies::birthdate.eq(update.birthdate),
        ))
        .execute(conn)
}

pub fn delete_baby_from_db(baby: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(babies::table.find(baby)).execute(conn)
}

pub fn get_all_babies_with_id(
    babies: Vec<i32>,
    pagination: Pagination,
) -> Result<(Vec<Baby>, u32), Error> {
    let conn = &mut establish_connection();
    babies::table
        .filter(babies::id.eq_any(babies))
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}
