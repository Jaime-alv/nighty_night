use chrono::NaiveDate;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{schema::babies, utils::datetime::format_date};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = babies)]
pub struct Baby {
    id: i32,
    name: String,
    birthdate: NaiveDate,
}

impl Baby {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub(crate) fn formatted_birthdate(&self) -> String {
        format_date(self.birthdate)
    }
}

#[derive(Insertable)]
#[diesel(table_name = babies)]
pub struct InsertableBaby {
    name: String,
    birthdate: NaiveDate,
}

impl InsertableBaby {
    pub fn new(name: String, birthdate: NaiveDate) -> Self {
        Self { name, birthdate }
    }
}
