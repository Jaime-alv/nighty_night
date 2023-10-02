use hyper::StatusCode;
use nighty_night::{
    data::query_dto::Pagination,
    service::{
        baby_service::{
            delete_baby_service, get_babies_for_user_service, get_baby_by_id_service,
            post_new_baby_service, post_share_baby_with_user_service, transfer_baby_service,
        },
        user_service::delete_user_from_database,
    },
};

use crate::{
    common::{
        assertions::{
            assert_compare_fields, assert_error_response, assert_len, assert_ok_message,
            assert_ok_paginated, assert_ok_response,
        },
        cte::DB_ERROR,
    },
    helper::{create_babies, delete_babies},
    mock::{entities::create_new_user, generate_new_baby},
};

pub mod common;
pub mod mock;

const BABIES_TO_ADD: i32 = 4;

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

    let response_get_baby_by_id = get_baby_by_id_service(test_baby_id).await;

    assert_ok_response(
        &response_get_baby_by_id,
        "Should load baby from database",
        StatusCode::OK,
    );

    let test_ingested_uid = response_get_baby_by_id
        .expect(DB_ERROR)
        .data
        .attributes
        .unique_id;

    assert_compare_fields(
        &test_baby_uid.to_string(),
        &test_ingested_uid.to_string(),
        "Unique id should match",
    );

    let babies = create_babies(BABIES_TO_ADD, user_id).await;
    let response_user_babies = get_babies_for_user_service(user_id, Pagination::default()).await;
    assert_ok_paginated(
        &response_user_babies,
        "Should load an array of babies",
        StatusCode::OK,
    );

    let new_babies_added = response_user_babies.expect(DB_ERROR).data;

    assert_len(
        &new_babies_added,
        BABIES_TO_ADD + 1,
        "User should have <X> babies",
    );

    let (test_user_two, _credentials) = create_new_user().await;

    let response_share_baby = post_share_baby_with_user_service(test_baby_id, test_user_two).await;

    assert_ok_message(
        &response_share_baby,
        "New user should be sharing baby",
        StatusCode::OK,
    );

    let test_babies_from_user_two =
        get_babies_for_user_service(test_user_two, Pagination::default())
            .await
            .expect(DB_ERROR);

    let record_baby = test_babies_from_user_two
        .data
        .first()
        .unwrap()
        .attributes
        .unique_id;
    assert_compare_fields(
        &test_baby_uid.to_string(),
        &record_baby.to_string(),
        "Unique id should match",
    );

    let response_transfer_baby = transfer_baby_service(test_baby_id, test_user_two).await;

    assert_ok_message(
        &response_transfer_baby,
        "Transfer baby should go Ok",
        StatusCode::OK,
    );

    let updated_babies_count = get_babies_for_user_service(user_id, Pagination::default())
        .await
        .expect(DB_ERROR)
        .data;

    assert_len(
        &updated_babies_count,
        BABIES_TO_ADD + 1,
        "Should maintain babies count",
    );

    let response_delete_baby = delete_baby_service(test_baby_id, user_id).await;
    assert_ok_message(
        &response_delete_baby,
        "Should delete baby from user",
        StatusCode::OK,
    );
    let updated_babies_count = get_babies_for_user_service(user_id, Pagination::default())
        .await
        .expect(DB_ERROR)
        .data;

    assert_len(
        &updated_babies_count,
        BABIES_TO_ADD,
        "Should loose one baby count",
    );

    let response_baby_id = get_baby_by_id_service(test_baby_id).await;
    assert_ok_response(
        &response_baby_id,
        "Baby should still exists",
        StatusCode::OK,
    );

    let response_delete_baby = delete_baby_service(test_baby_id, test_user_two).await;
    assert_ok_message(
        &response_delete_baby,
        "Should delete baby from database",
        StatusCode::OK,
    );

    let response_baby_id = get_baby_by_id_service(test_baby_id).await;
    assert_error_response(
        &response_baby_id,
        "Baby should not exists",
        StatusCode::NOT_FOUND,
    );

    // Tear down examples.
    delete_babies(babies, user_id).await;
    delete_user_from_database(user_id).expect(DB_ERROR);
    delete_user_from_database(test_user_two).expect(DB_ERROR);
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
