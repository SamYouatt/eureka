use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::helpers::{create_user_session, spawn_test_app};

#[tokio::test]
async fn can_view_idea() {
    // Arrange
    let test_app = spawn_test_app().await;

    create_user_session(&test_app).await;
    let idea_id = seed_idea(&test_app.db, "Test idea", "Just for testing").await;

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

async fn seed_idea(db: &PgPool, title: &str, tagline: &str) -> Uuid {
    let idea_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO ideas (id, title, tagline, created_at) VALUES ($1, $2, $3, $4)",
        idea_id,
        title.into(),
        tagline.into(),
        Utc::now()
    )
    .execute(db)
    .await
    .unwrap();

    idea_id
}
