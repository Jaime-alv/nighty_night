use chrono::NaiveDate;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;

use crate::{
    model::meals_model::{InsertableMeal, Meal},
    schema::meals,
};

use super::connection_psql::establish_async_connection;

pub async fn ingest_meal<T>(new_meal: T) -> Result<usize, Error>
where
    T: Into<InsertableMeal>,
{
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(meals::table)
        .values(new_meal.into())
        .execute(conn).await
}

pub async fn get_all_meals_from_baby(baby: i32) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_async_connection().await;
    meals::table.filter(meals::baby_id.eq(baby)).load(conn).await
}

pub async fn find_meals_by_date(baby: i32, date: NaiveDate) -> Result<Vec<Meal>, Error> {
    let top = date.and_hms_opt(23, 59, 59).unwrap();
    let down = date.and_hms_opt(0, 0, 1).unwrap();
    let conn = &mut establish_async_connection().await;
    meals::table
        .filter(meals::baby_id.eq(baby))
        .filter(meals::date.ge(down))
        .filter(meals::date.le(top))
        .load::<Meal>(conn).await
}
