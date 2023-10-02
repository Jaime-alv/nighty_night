use hyper::StatusCode;
use nighty_night::{
    data::query_dto::Pagination,
    service::{
        baby_service::{delete_baby_service, get_babies_for_user_service, post_new_baby_service},
        user_service::delete_user_from_database,
    },
};

use crate::{
    common::{
        assertions::{assert_len, assert_ok_message, assert_ok_paginated, assert_ok_response},
        cte::DB_ERROR,
    },
    helper::{create_babies, delete_babies},
    mock::{entities::create_new_user, generate_new_baby},
};

pub mod common;
pub mod mock;

#[ctor::ctor]
fn init() {
    common::initialiser::init()
}

#[tokio::test]
async fn test_baby_flow() {
    let (user_id, _user_credentials) = create_new_user().await;

    let baby = generate_new_baby();

    let response_new_baby = post_new_baby_service(baby, user_id).await;
    assert_ok_response(
        &response_new_baby,
        "Should create a new baby",
        StatusCode::CREATED,
    );

    let baby = response_new_baby.unwrap();
    let test_baby_id: i32 = baby.data.id;
    let test_baby_uid = baby.data.attributes.unique_id;

    let babies_to_add: i32 = 4;
    let babies = create_babies(babies_to_add, user_id).await;
    let response_user_babies = get_babies_for_user_service(user_id, Pagination::default()).await;
    assert_ok_paginated(
        &response_user_babies,
        "Should load an array of babies",
        StatusCode::OK,
    );

    let new_babies_added = response_user_babies.expect(DB_ERROR).data;

    assert_len(
        &new_babies_added,
        babies_to_add + 1,
        "User should have <X> babies",
    );

    let response_delete_baby = delete_baby_service(test_baby_id, user_id).await;
    assert_ok_message(
        &response_delete_baby,
        "Should delete baby from database",
        StatusCode::OK,
    );

    delete_babies(babies, user_id).await;
    delete_user_from_database(user_id).expect(DB_ERROR);
}

mod helper {
    use nighty_night::service::baby_service::delete_baby_service;

    use crate::{common::cte::DELETE, mock::entities::create_new_baby};

    pub(super) async fn create_babies(number: i32, user_id: i32) -> Vec<i32> {
        let mut babies_id: Vec<i32> = Vec::new();
        for _i in 0..number {
            let id = create_new_baby(user_id).await;
            babies_id.push(id)
        }
        babies_id
    }

    pub(super) async fn delete_babies(babies: Vec<i32>, user_id: i32) {
        for baby in babies {
            delete_baby_service(baby, user_id).await.expect(DELETE);
        }
    }
}
