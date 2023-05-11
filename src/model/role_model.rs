use diesel::{Selectable, Identifiable};

use crate::schema::roles;

#[derive(Selectable, Identifiable)]
#[diesel(table_name = roles)]
pub struct Role {
    id: i32,
    name: String
}
