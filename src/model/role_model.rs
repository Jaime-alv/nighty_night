use diesel::{Identifiable, Queryable, Selectable};

use crate::schema::roles;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = roles)]
pub struct Role {
    id: i16,
    name: String,
}

impl Role {
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}



#[derive(PartialEq, Clone, Debug)]
pub enum Rol {
    Anonymous,
    User,
    Admin,
}
