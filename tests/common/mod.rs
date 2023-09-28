pub mod alias;

use std::path::Path;

pub const VALUE_NONE: &'static str = "Some value expected, found None";
pub const DB_ERROR: &'static str = "Error connecting database";

pub fn init() {
    dotenvy::from_path(Path::new("./key/local.env")).unwrap();
}

pub fn compare_fields(original: &str, new: &str, msg: &str) {
    assert_eq!(
        original, new,
        "Test failed: {}. Expected: {} => Received: {}",
        msg, original, new
    )
}
