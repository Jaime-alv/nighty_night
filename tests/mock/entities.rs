use nighty_night::service::user_service::post_new_user_service;

use crate::common::cte::DB_ERROR;

use super::generate_new_user;

pub async fn create_new_user() -> i32 {
    let user = generate_new_user();
    let response_create_user = post_new_user_service(user).await;
    let (_new_created_user, id) = response_create_user.expect(DB_ERROR);
    id
}
