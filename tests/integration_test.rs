mod common;
mod import;
mod resources;
mod response;

use nighty_night::{configuration::settings::Setting, data::user_dto::NewUserDto};

use crate::{common::*, import::user::*, resources::user::*, response::*};

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
    test_created_response(&response_create_user, "Create new user");

    let (user, id) = response_create_user.expect(DB_ERROR);

    let response_load_profile = test_load_user_profile(id).await;

    test_ok_response(&response_load_profile, "Load user profile");

    let test_profile = test_load_user_profile(id).await.expect(DB_ERROR);

    compare_fields(
        &user.data.attributes.username,
        &test_profile.data.attributes.username,
        "User ingestion in db",
    );

    let response_patch_profile = test_patch_user_profile(id, &update_profile).await;

    test_ok_response(&response_patch_profile, "Update user profile");

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
}

#[tokio::test]
async fn test_user_creation() {
    let user = generate_new_user();
    test_created_response(&test_create_user(&user).await, "Create new user");
    let empty_user = NewUserDto {
        username: "".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_bad_request_response(&test_create_user(&empty_user).await, "Empty fields");

    let duplicate_user = NewUserDto {
        username: "guest".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    };
    test_bad_request_response(&test_create_user(&duplicate_user).await, "Duplicate user");
}
