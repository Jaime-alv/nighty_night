mod common;
mod import;
mod mock;

use fake::{faker::internet::en::Username, Fake};
use hyper::StatusCode;
use nighty_night::{
    configuration::settings::Setting,
    data::{
        common_structure::{SessionDto, UserDto},
        user_dto::NewUserDto,
    },
    response::{error::ApiError, response::RecordResponse},
};

use crate::{
    common::{record::*, *},
    import::user_service::*,
    mock::{
        generate_invalid_credentials, generate_login_credentials, generate_new_user,
        generate_update_user,
    },
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
    // let (user, update_profile) = tokio::join!(
    //     generate_new_user(),
    //     generate_update_user()
    // );
    let user = generate_new_user();
    let update_profile = generate_update_user();

    let response_create_user: Result<(RecordResponse<SessionDto>, i32), ApiError> =
        test_create_user(&user).await;
    assert_ok_response_id(
        &response_create_user,
        "Should create new user",
        StatusCode::CREATED,
    );

    let (new_created_user, id) = response_create_user.expect(DB_ERROR);

    let response_load_profile = test_load_user_profile(id).await;

    assert_ok_response(
        &response_load_profile,
        "Should load user profile",
        StatusCode::OK,
    );

    let test_profile = test_load_user_profile(id).await.expect(DB_ERROR);

    assert_compare_fields(
        &new_created_user.data.attributes.username,
        &test_profile.data.attributes.username,
        "Username should be the same",
    );

    let response_patch_profile: Result<RecordResponse<UserDto>, ApiError> =
        test_patch_user_profile(id, &update_profile).await;

    assert_ok_response(
        &response_patch_profile,
        "Should update user profile",
        StatusCode::OK,
    );

    // Check updated fields
    let updated_fields_user = test_load_user_profile(id)
        .await
        .expect(DB_ERROR)
        .data
        .attributes;
    assert_compare_fields(
        &update_profile.email.expect(VALUE_NONE),
        &updated_fields_user.email.expect(VALUE_NONE),
        "Should update email",
    );

    // Login user
    let login_credentials = generate_login_credentials(&user.username, &user.password);
    let response_login = test_login_service(login_credentials).await;
    assert_ok_response_id(&response_login, "Should login user in", StatusCode::OK);
}

#[tokio::test]
async fn test_user_creation() {
    let user = generate_new_user();
    assert_ok_response_id(
        &test_create_user(&user).await,
        "Should create a new user",
        StatusCode::CREATED,
    );
    let empty_user = NewUserDto {
        username: "".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    assert_error_response_id(
        &test_create_user(&empty_user).await,
        "Should not create user, empty fields.",
        StatusCode::BAD_REQUEST,
    );

    let duplicate_user = NewUserDto {
        username: "guest".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    assert_error_response_id(
        &test_create_user(&duplicate_user).await,
        "Should not create user, duplicate user.",
        StatusCode::BAD_REQUEST,
    );

    let spaces_user = NewUserDto {
        username: " ".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    assert_error_response_id(
        &test_create_user(&spaces_user).await,
        "Should not create user, username contains spaces",
        StatusCode::BAD_REQUEST,
    );

    let invalid_password = NewUserDto {
        username: Username().fake(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    assert_error_response_id(
        &test_create_user(&invalid_password).await,
        "Should not create user, password is too short",
        StatusCode::BAD_REQUEST,
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

#[tokio::test]
async fn test_login_user() {
    let user = generate_new_user();
    test_create_user(&user).await.expect(DB_ERROR);

    let valid_credentials = generate_login_credentials(&user.username, &user.password);
    let response = test_login_service(valid_credentials).await;
    assert_ok_response_id(&response, "User should login into system", StatusCode::OK);

    let invalid_password = generate_invalid_credentials(Some(&user.username), None);
    let invalid_response = test_login_service(invalid_password).await;
    assert_error_response_id(
        &invalid_response,
        "Password should not match",
        StatusCode::BAD_REQUEST,
    );

    let invalid_user = generate_invalid_credentials(None, Some(&user.password));
    let invalid_user_response = test_login_service(invalid_user).await;
    assert_error_response_id(
        &invalid_user_response,
        "User should not exist",
        StatusCode::BAD_REQUEST,
    );

    let invalid_credentials = generate_invalid_credentials(None, None);
    let all_field_invalid_response = test_login_service(invalid_credentials).await;
    assert_error_response_id(
        &all_field_invalid_response,
        "Neither user nor password should exist",
        StatusCode::BAD_REQUEST,
    );

    // TODO: User is not active
}
