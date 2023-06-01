use diesel::{prelude::*, result::Error};


use crate::{model::meals_model::{InsertableMeal, Meal}, schema::meals};

use super::connection_psql::establish_connection;

pub fn ingest_meal<T>(new_meal: T) -> Result<usize, Error>
where
    T: Into<InsertableMeal>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(meals::table)
        .values(new_meal.into())
        .execute(conn)
}

pub fn get_all_meals_from_baby(baby: i32) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_connection();    
    meals::table.filter(meals::baby_id.eq(baby)).load(conn)
}