mod common;
mod import;
mod mock;

use fake::{faker::internet::en::Username, Fake};
use hyper::StatusCode;
use nighty_night::{configuration::settings::Setting, data::user_dto::NewUserDto};

use crate::{
    common::{record::*, *},
    import::user_service::*,
    mock::{generate_new_user, generate_update_user},
};

#[ctor::ctor]
fn init() {
    common::init()
}

#[test]
fn test_branch() {
    assert_eq!(Setting::Branch.get(), "local");
    assert_eq!(Setting::RedisHost.get(), "redis://127.0.0.1:6379/");
}

#[tokio::test]
async fn test_user_flow() {
    // Create new user
    let user = generate_new_user();
    let update_profile = generate_update_user();

    let response_create_user = test_create_user(&user).await;
    test_created_user_response(&response_create_user, "Create new user");

    let (user, id) = response_create_user.expect(DB_ERROR);

    let response_load_profile = test_load_user_profile(id).await;

    test_ok_response(&response_load_profile, "Load user profile", StatusCode::OK);

    let test_profile = test_load_user_profile(id).await.expect(DB_ERROR);

    compare_fields(
        &user.data.attributes.username,
        &test_profile.data.attributes.username,
        "User ingestion in db",
    );

    let response_patch_profile = test_patch_user_profile(id, &update_profile).await;

    test_ok_response(
        &response_patch_profile,
        "Update user profile",
        StatusCode::OK,
    );

    // Check updated fields
    let updated_fields_user = test_load_user_profile(id)
        .await
        .expect(DB_ERROR)
        .data
        .attributes;
    compare_fields(
        &update_profile.email.expect(VALUE_NONE),
        &updated_fields_user.email.expect(VALUE_NONE),
        "Update user information profile",
    );

    //
}

#[tokio::test]
async fn test_user_creation() {
    let user = generate_new_user();
    test_created_user_response(&test_create_user(&user).await, "Create new user");
    let empty_user = NewUserDto {
        username: "".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_error_created_user_response(&test_create_user(&empty_user).await, "Empty fields");

    let duplicate_user = NewUserDto {
        username: "guest".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_error_created_user_response(&test_create_user(&duplicate_user).await, "Duplicate user");

    let spaces_user = NewUserDto {
        username: " ".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_error_created_user_response(
        &test_create_user(&spaces_user).await,
        "Username contains spaces",
    );

    let invalid_password = NewUserDto {
        username: Username().fake(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_error_created_user_response(
        &test_create_user(&invalid_password).await,
        "Password too short",
    );
}

#[test]
fn test_validation_user_info() {
    let error_user = NewUserDto {
        username: "admin".to_string(),
        password: "abcd".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    let good_user = NewUserDto {
        username: "adminInvent".to_string(),
        password: "abcd".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    let empty_fields = NewUserDto {
        username: "".to_string(),
        password: "abcd".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    assert!(test_validate_user_fields(error_user).is_err());
    assert!(test_validate_user_fields(good_user).is_ok());
    assert!(test_validate_user_fields(empty_fields).is_err());
}