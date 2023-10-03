use nighty_night::{service::{user_service::post_new_user_service, baby_service::post_new_baby_service}, data::user_dto::LoginDto};

use crate::common::cte::DB_ERROR;

use super::{generate_new_user, generate_new_baby};

pub async fn create_new_user() -> (i32, LoginDto) {
    let user = generate_new_user();
    let user_credentials = LoginDto {
        username: user.username.to_string(),
        password: user.password.to_string(),
    };
    let response_create_user = post_new_user_service(user).await;
    let (_new_created_user, id) = response_create_user.expect(DB_ERROR);
    (id, user_credentials)
}

pub async fn create_new_baby(user_id: i32) -> i32 {
    let baby = generate_new_baby();
    let new_baby = post_new_baby_service(baby, user_id).await.expect(DB_ERROR);
    new_baby.data.id
}
