use crate::schema::{babies, dreams, meals, roles, users, users_roles, weights};
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

use super::connection_psql::establish_connection;

#[derive(Serialize)]
pub struct StatsDB<'a> {
    pub users: TableDescription<'a>,
    pub babies: TableDescription<'a>,
    pub dreams: TableDescription<'a>,
    pub meals: TableDescription<'a>,
    pub weights: TableDescription<'a>,
}

#[derive(Serialize)]
pub struct TableDescription<'a> {
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
        users: TableDescription {
            name: "users",
            value: users,
        },
        babies: TableDescription {
            name: "babies",
            value: babies,
        },
        dreams: TableDescription {
            name: "dreams",
            value: dreams,
        },
        meals: TableDescription {
            name: "meals",
            value: meals,
        },
        weights: TableDescription {
            name: "weights",
            value: weights,
        },
    };
    Ok(result)
}

pub struct GroupedRole {
    pub id: i16,
    pub name: String,
    pub count: i64,
}

/// Select all roles and count how many of each there are.
///
/// Raw SQL:
/// ```SQL
/// SELECT roles.id, roles.name, COUNT(roles.id)
///     from roles
/// INNER JOIN users_roles ON roles.id = users_roles.rol_id
/// GROUP BY roles.id;
/// ```
pub fn select_roles_and_group_by_count() -> Result<Vec<GroupedRole>, Error> {
    let conn = &mut establish_connection();
    let data = roles::table
        .inner_join(users_roles::table.on(users_roles::rol_id.eq(roles::id)))
        .group_by(roles::id)
        .select((roles::id, roles::name, count(roles::id)))
        .load::<(i16, String, i64)>(conn)?;
    Ok(data
        .into_iter()
        .map(|(id, name, count)| GroupedRole { id, name, count })
        .collect())
}
