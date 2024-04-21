use sqlx::types::Uuid;
use chrono::Utc;
use sqlx::PgPool;
use wiremock::{
    matchers::{method, path},
    Mock, MockGuard, ResponseTemplate,
};

use crate::helpers::{spawn_test_app, assert_redirect_to, TestApp};

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

#[tokio::test]
async fn on_login_existing_user_not_added_to_db() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let _mock_open_id = configure_open_id_mock(&test_app).await;

    seed_user(&test_app.db, "test@test.com").await;

    // Act
    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());

    let created_user = sqlx::query!(
        "SELECT COUNT(*) FROM users WHERE email = $1",
        "test@test.com"
    )
    .fetch_one(&test_app.db)
    .await
    .expect("Failed to fetch new user");

    assert_eq!(created_user.count, Some(1));
}

#[tokio::test]
async fn on_login_should_insert_session_to_db() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let _mock_open_id = configure_open_id_mock(&test_app).await;

    let user_id = seed_user(&test_app.db, "test@test.com").await;

    // Act
    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());

    let created_user = sqlx::query!(
        "SELECT COUNT(*) FROM sessions WHERE user_id = $1",
        user_id,
    )
    .fetch_one(&test_app.db)
    .await
    .expect("Failed to fetch new user");

    assert_eq!(created_user.count, Some(1));
}

#[tokio::test]
async fn without_session_should_redircect_to_login() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();

    // Act
    let url = format!("{}/", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_redirect_to(&response, "/login");
}

async fn seed_user(db: &PgPool, email: &str) -> Uuid {
    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, email, created_at) VALUES ($1, $2, $3)",
        user_id,
        email,
        Utc::now(),
    )
    .execute(db)
    .await
    .unwrap();

    user_id
}
