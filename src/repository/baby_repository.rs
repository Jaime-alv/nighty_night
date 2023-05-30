use diesel::prelude::*;
use diesel::result::Error;

use crate::{data::baby_dto::NewBabyDto, model::baby_model::Baby, schema::babies};

use super::connection_psql::establish_connection;


pub fn ingest_new_baby_in_db(
    new_user: NewBabyDto,
) -> Result<Baby, Error> {
    let conn = &mut establish_connection();
    diesel::insert_into(babies::table)
        .values(new_user)
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