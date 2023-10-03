pub mod common;

use axum::http::StatusCode;
use axum_test_helper::{TestClient, TestResponse};
use nighty_night::app::create_app_route;
use serde_json::{json, Value};

const SESSION: &'static str = "/api/auth/session";

enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HttpMethod {
    fn as_str(self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

#[ctor::ctor]
fn init() {
    common::initialiser::init()
}

#[tokio::test]
async fn test_anonymous_call() {
    let router = create_app_route().await;
    let client = TestClient::new(router);

    let welcome_test = client.get(SESSION).send().await;

    assert_ok_status(&welcome_test, StatusCode::OK, HttpMethod::GET, SESSION);

    let guest_user = json!({"data": {"attributes": {"baby_info": [], "username": "guest"}, "id": 1, "type": "user"}});

    assert_eq!(
        welcome_test.json::<Value>().await,
        guest_user,
        "\nShould return Guest user"
    );
}

fn assert_ok_status(
    received_status: &TestResponse,
    expected_code: StatusCode,
    method: HttpMethod,
    msg: &str,
) {
    assert_eq!(
        received_status.status(),
        expected_code,
        "\nTest Failed: {} {}\nExpected code: {} => Received code: {}",
        method.as_str(),
        msg,
        expected_code,
        received_status.status()
    )
}
