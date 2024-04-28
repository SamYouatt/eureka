use chrono::{DateTime, Duration, Utc};
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::helpers::{
    assert_redirect_to, configure_open_id_mock, create_user_session, spawn_test_app,
};

#[tokio::test]
async fn oauth_callback_attaches_cookie() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    let _mock_open_id = configure_open_id_mock("test@test.com", &test_app).await;

    // Act
    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    let cookie = response.cookies().find(|cookie| cookie.name() == "sid");
    println!("{:?}", cookie);

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert!(response.cookies().any(|cookie| cookie.name() == "sid"));
}

#[tokio::test]
async fn on_login_new_user_added_to_db() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let _mock_open_id = configure_open_id_mock("test@test.com", &test_app).await;

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
    let _mock_open_id = configure_open_id_mock("test@test.com", &test_app).await;

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
    let _mock_open_id = configure_open_id_mock("test@test.com", &test_app).await;

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

    let created_user = sqlx::query!("SELECT COUNT(*) FROM sessions WHERE user_id = $1", user_id,)
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch new user");

    assert_eq!(created_user.count, Some(1));
}

#[tokio::test]
async fn without_session_should_redircect_to_login() {
    // Arrange
    let test_app = spawn_test_app().await;

    // Act
    let url = format!("{}/", test_app.address);
    let response = test_app
        .client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_redirect_to(&response, "/login");
}

#[tokio::test]
async fn expired_session_should_redirect_to_login() {
    // Arrange
    let test_app = spawn_test_app().await;
    let user_id = create_user_session("test@test.com", &test_app).await;
    let session_id = find_user_session(&test_app.db, user_id).await;

    let expired_token_age = Utc::now() - Duration::hours(2);
    expire_session(&test_app.db, &session_id, expired_token_age).await;

    // Act
    let url = format!("{}/", test_app.address);
    let response = test_app
        .client
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

async fn find_user_session(db: &PgPool, user_id: Uuid) -> String {
    sqlx::query!(
        "SELECT session_id FROM sessions WHERE user_id = $1 LIMIT 1",
        user_id,
    )
    .fetch_one(db)
    .await
    .unwrap()
    .session_id
}

async fn expire_session(db: &PgPool, session_id: &str, expires_at: DateTime<Utc>) {
    sqlx::query!(
        "UPDATE sessions SET expires_at = $1 WHERE session_id = $2",
        expires_at,
        session_id,
    )
    .execute(db)
    .await
    .unwrap();
}
