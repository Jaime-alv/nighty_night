pub mod cte {
    pub const VALUE_NONE: &'static str = "Some value expected, found None";
    pub const DB_ERROR: &'static str = "Error connecting database";
    pub const NO_USER_ERROR: &'static str = "No user found.";
}

pub mod initialiser {
    use std::path::Path;

    pub fn init() {
        dotenvy::from_path(Path::new("./key/local.env")).unwrap();
    }
}

pub mod assertions {
    use hyper::StatusCode;
    use nighty_night::response::{
        error::ApiError,
        response::{MsgResponse, PagedResponse, RecordResponse},
    };
    use serde::Serialize;
    use std::fmt::Debug;

    pub fn assert_compare_fields(original: &str, new: &str, msg: &str) {
        assert_eq!(
            original, new,
            "Test failed: {}. Expected: {} => Received: {}",
            msg, original, new
        )
    }

    pub fn assert_ok_response<T>(
        response: &Result<RecordResponse<T>, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_ok(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap_err().get_error().0.as_u16(),
        )
    }

    pub fn assert_error_response<T>(
        response: &Result<RecordResponse<T>, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_err(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap().status_code,
        )
    }

    pub fn assert_ok_response_id<T>(
        response: &Result<(RecordResponse<T>, i32), ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_ok(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap_err().get_error().0.as_u16(),
        )
    }

    pub fn assert_error_response_id<T>(
        response: &Result<(RecordResponse<T>, i32), ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_err(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap().0.status_code,
        )
    }

    pub fn assert_ok_message(
        response: &Result<MsgResponse, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) {
        assert!(
            response.is_ok(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap_err().get_error().0,
        )
    }

    pub fn assert_error_message(
        response: &Result<MsgResponse, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) {
        assert!(
            response.is_err(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap().get_response().0,
        )
    }

    pub fn assert_ok_paginated<T>(
        response: &Result<PagedResponse<T>, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_ok(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap_err().get_error().0,
        )
    }

    pub fn assert_error_paginated<T>(
        response: &Result<PagedResponse<T>, ApiError>,
        msg: &str,
        expected_status: StatusCode,
    ) where
        T: Serialize + Debug,
    {
        assert!(
            response.is_err(),
            "Test failed: {}. Expected: {} => Received: {}",
            msg,
            expected_status,
            response.as_ref().unwrap().status_code
        )
    }
}
