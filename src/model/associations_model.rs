use diesel::prelude::*;

use crate::schema::{users_roles, users_babies};

use super::{user_model::{User}, baby_model::Baby, role_model::Role};

#[derive(Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = rol_id))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    id: i32,
    rol_id: i32,
    user_id: i32
}

#[derive(Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Baby, foreign_key = baby_id))]
#[diesel(table_name = users_babies)]
pub struct UserBaby {
    id: i32,
    baby_id: i32,
    user_id: i32
}