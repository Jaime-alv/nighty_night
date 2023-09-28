use nighty_night::{
    data::{
        common_structure::{SessionDto, UserDto},
        user_dto::{NewUserDto, UpdateUserDto},
    },
    response::{error::ApiError, response::RecordResponse},
    service::user_service::{get_user_by_id_service, patch_user_service, post_new_user_service},
};

pub async fn test_create_user(
    user: &NewUserDto,
) -> Result<(RecordResponse<SessionDto>, i32), ApiError> {
    let new_user = NewUserDto {
        username: user.username.to_owned(),
        password: user.password.to_owned(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        surname: user.surname.to_owned(),
    };
    post_new_user_service(new_user).await
}

pub async fn test_load_user_profile(user_id: i32) -> Result<RecordResponse<UserDto>, ApiError> {
    get_user_by_id_service(user_id).await
}

pub async fn test_patch_user_profile(
    user_id: i32,
    update_profile: &UpdateUserDto,
) -> Result<RecordResponse<UserDto>, ApiError> {
    let update_fields = UpdateUserDto {
        email: update_profile.email.to_owned(),
        name: update_profile.name.to_owned(),
        surname: update_profile.surname.to_owned(),
    };

    let response = patch_user_service(user_id, update_fields).await;
    response
}
