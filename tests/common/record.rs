use std::fmt::Debug;

use hyper::StatusCode;
use nighty_night::response::{error::ApiError, response::RecordResponse};
use serde::Serialize;

pub fn test_ok_response<T>(
    response: &Result<RecordResponse<T>, ApiError>,
    msg: &str,
    expected_status: StatusCode,
) where
    T: Serialize + Debug,
{
    let code: u16 = expected_status.as_u16();
    assert!(
        response.is_ok(),
        "Test failed: {}. Expected: {} => Received: {}",
        msg,
        code,
        response.as_ref().unwrap_err().get_error().0.as_u16(),
    )
}

pub fn test_error_response<T>(
    response: &Result<RecordResponse<T>, ApiError>,
    msg: &str,
    expected_status: StatusCode,
) where
    T: Serialize + Debug,
{
    let code: u16 = expected_status.as_u16();
    assert!(
        response.is_err(),
        "Test failed: {}. Expected: {} => Received: {}",
        msg,
        code,
        response.as_ref().unwrap().status_code,
    )
}

pub fn test_created_user_response<T>(
    response: &Result<(RecordResponse<T>, i32), ApiError>,
    msg: &str,
) where
    T: Serialize + Debug,
{
    let code: u16 = 201;
    assert!(
        response.is_ok(),
        "Test failed: {}. Expected: {} => Received: {}",
        msg,
        code,
        response.as_ref().unwrap_err().get_error().0.as_u16(),
    )
}

pub fn test_error_created_user_response<T>(
    response: &Result<(RecordResponse<T>, i32), ApiError>,
    msg: &str,
) where
    T: Serialize + Debug,
{
    let code: u16 = 400;
    assert!(
        response.is_err(),
        "Test failed: {}. Expected: {} => Received: {}",
        msg,
        code,
        response.as_ref().unwrap().0.status_code,
    )
}
