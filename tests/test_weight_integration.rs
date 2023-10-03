use crate::common::{assertions::assert_ok_response, cte::DB_ERROR};
use fake::{Fake, Faker};
use hyper::StatusCode;
use mock::generate_date;
use nighty_night::service::user_service::delete_user_from_database;
use nighty_night::service::weight_service::patch_weight_service;
use nighty_night::{
    data::weight_dto::InputWeightDto,
    service::{baby_service::delete_baby_service, weight_service::post_weight_service},
};

use crate::mock::entities::{create_new_baby, create_new_user};

pub mod common;
pub mod mock;

#[ctor::ctor]
fn init() {
    common::initialiser::init()
}

#[tokio::test]
async fn test_weight_flow() {
    let (user_id, _user_credentials) = create_new_user().await;
    let baby_id = create_new_baby(user_id).await;

    let response_post_weight = post_weight_service(generate_weight(), baby_id).await;
    assert_ok_response(
        &response_post_weight,
        "Should add a new measure",
        StatusCode::CREATED,
    );

    let response_data = response_post_weight.expect(DB_ERROR).data;
    assert_count_decimals_is(response_data.attributes.value);

    delete_baby_service(baby_id, user_id).await.expect(DB_ERROR);
    delete_user_from_database(user_id).expect(DB_ERROR);
}

#[tokio::test]
async fn patch_weight_value() {
    let (user_id, _user_credentials) = create_new_user().await;
    let baby_id = create_new_baby(user_id).await;

    let added_weight = post_weight_service(generate_weight(), baby_id)
        .await
        .expect(DB_ERROR)
        .data;
    let weight_id: i32 = added_weight.id;
    let one_decimal = InputWeightDto {
        date: None,
        value: Some(1.2),
    };

    let new_weight_value: &f32 = &one_decimal.value.unwrap();

    let response_patch_weight = patch_weight_service(one_decimal, weight_id, baby_id).await;

    assert_ok_response(
        &response_patch_weight,
        "Should patch weight",
        StatusCode::OK,
    );

    let patched_weight: &f32 = &response_patch_weight
        .as_ref()
        .expect(DB_ERROR)
        .data
        .attributes
        .value;

    assert_eq!(new_weight_value, patched_weight, "Should be the same value");

    assert_count_decimals_is(*patched_weight);

    assert_eq!(
        added_weight.attributes.date,
        response_patch_weight.expect(DB_ERROR).data.attributes.date,
        "Dates should be the same"
    );

    let two_decimals = InputWeightDto {
        date: None,
        value: Some(1.23),
    };

    let data_two_decimals = patch_weight_service(two_decimals, weight_id, baby_id)
        .await
        .expect(DB_ERROR)
        .data;

    assert_count_decimals_is(data_two_decimals.attributes.value);
    
    delete_baby_service(baby_id, user_id).await.expect(DB_ERROR);
    delete_user_from_database(user_id).expect(DB_ERROR);
}

#[test]
fn test_count_decimals() {
    let value_one: f32 = 1.1;
    assert_count_decimals_is(value_one);
    let value_two: f32 = 1.12;
    assert_count_decimals_is(value_two);
    let value_three: f32 = 1.123;
    assert_count_decimals_is(value_three);
}

fn generate_weight() -> InputWeightDto {
    InputWeightDto {
        date: Some(generate_date()),
        value: Some(Faker.fake::<f32>()),
    }
}

fn assert_count_decimals_is(value: f32) {
    let value = value.to_string().split(".").last().unwrap().len();
    assert!(
        value.le(&3),
        "Value should have three or less decimals. Received: {} => Expected: 3",
        value
    );
}
