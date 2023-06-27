use super::connection_psql::establish_connection;
use crate::{
    model::{associations_model::UserBaby, baby_model::Baby, user_model::User},
    schema::{babies, users_babies},
};
use diesel::prelude::*;

// #[axum_macros::debug_handler]
pub fn find_related_babies(user: &User) -> Vec<Baby> {
    let conn = &mut establish_connection();
    let baby_id = UserBaby::belonging_to(user).select(users_babies::baby_id);
    babies::table
        .filter(babies::id.eq_any(baby_id))
        .load::<Baby>(conn)
        .expect("could not load babies from user.")
}
