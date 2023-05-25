use diesel::{Queryable, Selectable, Identifiable};


use crate::{schema::babies};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = babies)]
pub struct Baby {
    id: i32,
    name: String,
}

impl Baby {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}
