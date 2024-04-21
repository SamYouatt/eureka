use sqlx::types::Uuid;
use chrono::Utc;
use sqlx::PgPool;
use wiremock::{
    matchers::{method, path},
    Mock, MockGuard, ResponseTemplate,
};

use crate::helpers::{spawn_test_app, TestApp};

#[derive(serde::Serialize)]
struct UseInfoResponse {
    email: String,
}

#[derive(serde::Serialize)]
struct AuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl AuthTokenResponse {
    fn new() -> Self {
        Self {
            access_token: "test_access_token".into(),
            token_type: "basic".into(),
            expires_in: 3600,
        }
    }
}

async fn configure_open_id_mock(test_app: &TestApp) -> (MockGuard, MockGuard) {
    // These routes are mocked and not the real routes used by Google
    let auth_token_response = ResponseTemplate::new(200).set_body_json(AuthTokenResponse::new());
    let mock_token_exchange = Mock::given(path("/token"))
        .and(method("POST"))
        .respond_with(auth_token_response)
        .named("OpenId auth token exchange")
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    let user_info_response = ResponseTemplate::new(200).set_body_json(UseInfoResponse {
        email: "test@test.com".into(),
    });
    let mock_user_info = Mock::given(path("/user"))
        .and(method("GET"))
        .respond_with(user_info_response)
        .named("OpenId user info")
        .expect(1)
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    (mock_token_exchange, mock_user_info)
}

#[tokio::test]
async fn oauth_callback_attaches_cookie() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    let _mock_open_id = configure_open_id_mock(&test_app).await;

    // Act
    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert!(response
        .cookies()
        .find(|cookie| cookie.name() == "sid")
        .is_some());
}

#[tokio::test]
async fn on_login_new_user_added_to_db() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let _mock_open_id = configure_open_id_mock(&test_app).await;

    // Act
    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());

    let created_user = sqlx::query!("SELECT email FROM users WHERE email = $1", "test@test.com")
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch new user");

    assert_eq!(created_user.email, "test@test.com");
}