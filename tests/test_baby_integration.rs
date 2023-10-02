use hyper::StatusCode;
use nighty_night::{
    data::query_dto::Pagination,
    service::{
        baby_service::{get_babies_for_user_service, post_new_baby_service},
        user_service::delete_user_from_database,
    },
};

use crate::{
    common::{
        assertions::{assert_ok_paginated, assert_ok_response},
        cte::DB_ERROR,
    },
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
    let user_id = create_new_user().await;

    let baby = generate_new_baby();

    let response_new_baby = post_new_baby_service(baby, user_id).await;
    assert_ok_response(
        &response_new_baby,
        "Should create a new baby",
        StatusCode::CREATED,
    );

    let baby = response_new_baby.unwrap();
    let baby_id: i32 = baby.data.id;
    let baby_uid = baby.data.attributes.unique_id;

    let response_user_babies =
        get_babies_for_user_service(user_id.into(), Pagination::default()).await;
    assert_ok_paginated(
        &response_user_babies,
        "Should load an array of babies",
        StatusCode::OK,
    );

    delete_user_from_database(user_id).expect(DB_ERROR);
}
