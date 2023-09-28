use std::fmt::{Debug, Display};

use nighty_night::response::error::ApiError;
use serde::Serialize;

fn test_status_code<T>(response: &Result<T, ApiError>, msg: &str, code: u16)
where
    T: Debug,
{
    assert!(
        response.is_ok(),
        "Test failed: {}. Expected: {} => Received: {}",
        msg,
        code,
        response.as_ref().unwrap_err().get_error().0.as_u16()
    )
}

fn test_error_status_code<T>(response: &Result<T, ApiError>, msg: &str, code: u16)
where
    T: Debug,
{
    assert!(
        response.is_err(),
        "Test failed: {}. Expected: {} => Received: {:#?}",
        msg,
        code,
        response.as_ref().unwrap()
    )
}

pub fn test_created_response<T>(response: &Result<T, ApiError>, msg: &str)
where
    T: Debug,
{
    let code: u16 = 201;
    test_status_code(response, msg, code)
}

pub fn test_ok_response<T>(response: &Result<T, ApiError>, msg: &str)
where
    T: Debug,
{
    let code: u16 = 200;
    test_status_code(response, msg, code)
}

pub fn test_bad_request_response<T>(response: &Result<T, ApiError>, msg: &str)
where
    T: Debug,
{
    let code: u16 = 400;
    test_error_status_code(response, msg, code)
}
