use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::helpers::{create_user_session, seed_user, spawn_test_app};

#[tokio::test]
async fn can_view_idea() {
    // Arrange
    let test_app = spawn_test_app().await;

    let user_id = create_user_session("test@test.com", &test_app).await;
    let idea_id = seed_idea(&test_app.db, "Test idea", "Just for testing", user_id).await;

    // Act
    let url = format!("{}/ideas/{}", test_app.address, idea_id);
    let response = test_app
        .client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let response_body = response.text().await.unwrap();
    assert_eq!(1, response_body.match_indices("Test idea").count());
    assert_eq!(1, response_body.match_indices("Just for testing").count());
}

#[tokio::test]
async fn cannot_view_another_user_idea() {
    // Arrange
    let test_app = spawn_test_app().await;
    let alice_user = Uuid::new_v4();
    seed_user(alice_user, "alice@test.com", &test_app.db).await;
    let idea_id = seed_idea(&test_app.db, "Alice's idea", "Just for testing", alice_user).await;

    let _bob_user = create_user_session("bob@test.com", &test_app).await;

    // Act
    let url = format!("{}/ideas/{}", test_app.address, idea_id);
    let response = test_app
        .client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 401);
}

async fn seed_idea(db: &PgPool, title: &str, tagline: &str, user_id: Uuid) -> Uuid {
    let idea_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO ideas (id, title, tagline, created_at, user_id) VALUES ($1, $2, $3, $4, $5)",
        idea_id,
        title.into(),
        tagline.into(),
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .unwrap();

    idea_id
}
