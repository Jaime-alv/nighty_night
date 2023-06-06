use diesel::{prelude::*, result::Error};

use crate::{
    model::dream_model::{Dream, InsertableDream},
    schema::dreams,
};

use super::connection_psql::establish_connection;

pub fn ingest_new_dream<T>(new_dream: T) -> Result<usize, Error>
where
    T: Into<InsertableDream>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(dreams::table)
        .values(new_dream.into())
        .execute(conn)
}

pub fn get_all_dreams_from_baby(baby: i32) -> Result<Vec<Dream>, Error> {
    let conn = &mut establish_connection();
    dreams::table.filter(dreams::baby_id.eq(baby)).load(conn)
}

/// Filter table dreams by baby_id, where to_date is null and order
/// in descending to get the higher one.
pub fn get_last_dream(baby: i32) -> Result<Dream, Error> {
    let conn = &mut establish_connection();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::to_date.is_null())
        .order(dreams::from_date.desc())
        .first(conn)
}

pub fn update_last_dream(dream: InsertableDream) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    let last_dream = get_last_dream(dream.baby_id());
    diesel::update(dreams::table.filter(dreams::id.eq(last_dream.unwrap().id())))
        .set(dreams::to_date.eq(dream.to_date()))
        .execute(conn)
}
