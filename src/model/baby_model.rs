use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::{
    schema::babies,
    utils::datetime::{format_date, now},
};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = babies)]
pub struct Baby {
    id: i32,
    unique_id: Uuid,
    name: String,
    birthdate: NaiveDate,
    belongs_to: i32,
    added_on: NaiveDateTime,
}

impl Baby {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn birthdate(&self) -> NaiveDate {
        self.birthdate
    }

    pub(crate) fn formatted_birthdate(&self) -> String {
        format_date(self.birthdate)
    }

    pub fn belongs_to(&self) -> i32 {
        self.belongs_to
    }

    pub fn added_on(&self) -> NaiveDateTime {
        self.added_on
    }

    pub fn formatted_added_on(&self) -> String {
        format_date(self.added_on.date())
    }

    pub fn unique_id(&self) -> Uuid {
        self.unique_id
    }
}

#[derive(Insertable)]
#[diesel(table_name = babies)]
pub struct InsertableBaby {
    name: String,
    unique_id: Uuid,
    birthdate: NaiveDate,
    belongs_to: i32,
    added_on: NaiveDateTime,
}

impl InsertableBaby {
    pub fn new(name: String, birthdate: NaiveDate, user_id: i32) -> Self {
        Self {
            name,
            unique_id: Uuid::new_v4(),
            birthdate,
            belongs_to: user_id,
            added_on: now(),
        }
    }
}
