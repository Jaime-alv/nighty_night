use crate::schema::{babies, dreams, meals, users, weights};
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

use super::connection_psql::establish_connection;

#[derive(Serialize)]
pub struct StatsDB<'a> {
    pub users: UsersTable<'a>,
    pub babies: BabiesTable<'a>,
    pub dreams: DreamsTable<'a>,
    pub meals: MealsTable<'a>,
    pub weights: WeightsTable<'a>,
}

#[derive(Serialize)]
pub struct UsersTable<'a> {
    name: &'a str,
    value: i64,
}
#[derive(Serialize)]
pub struct BabiesTable<'a> {
    name: &'a str,
    value: i64,
}
#[derive(Serialize)]
pub struct DreamsTable<'a> {
    name: &'a str,
    value: i64,
}
#[derive(Serialize)]
pub struct MealsTable<'a> {
    name: &'a str,
    value: i64,
}
#[derive(Serialize)]
pub struct WeightsTable<'a> {
    name: &'a str,
    value: i64,
}

pub fn count_records() -> Result<StatsDB<'static>, Error> {
    let conn = &mut establish_connection();
    let users: i64 = users::table.select(users::id).count().get_result(conn)?;
    let babies: i64 = babies::table.select(babies::id).count().get_result(conn)?;
    let dreams: i64 = dreams::table.select(dreams::id).count().get_result(conn)?;
    let meals: i64 = meals::table.select(meals::id).count().get_result(conn)?;
    let weights: i64 = weights::table
        .select(weights::id)
        .count()
        .get_result(conn)?;
    let result = StatsDB {
        users: UsersTable {
            name: "users",
            value: users,
        },
        babies: BabiesTable {
            name: "babies",
            value: babies,
        },
        dreams: DreamsTable {
            name: "dreams",
            value: dreams,
        },
        meals: MealsTable {
            name: "meals",
            value: meals,
        },
        weights: WeightsTable {
            name: "weights",
            value: weights,
        },
    };
    Ok(result)
}
