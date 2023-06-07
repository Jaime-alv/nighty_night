use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::babies;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = babies)]
pub struct NewBabyDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct BabyDto {
    pub id: i32,
    pub name: String,
}