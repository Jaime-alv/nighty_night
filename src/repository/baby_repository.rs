use diesel::prelude::*;
use diesel::result::Error;

use crate::{
    data::baby_dto::UpdateBaby,
    model::baby_model::{Baby, InsertableBaby},
    schema::babies,
};

use super::connection_psql::establish_connection;

pub fn ingest_new_baby_in_db<T>(new_baby: T) -> Result<Baby, Error>
where
    T: Into<InsertableBaby>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(babies::table)
        .values(new_baby.into())
        .returning(Baby::as_returning())
        .get_result(conn)

    // .execute(conn)
}

pub fn load_baby_by_id(id: i32) -> Result<Baby, Error> {
    let conn = &mut establish_connection();
    babies::table.find(id).first(conn)
}

pub fn query_babies() -> Result<Vec<Baby>, Error> {
    let conn = &mut establish_connection();
    babies::table.load(conn)
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
