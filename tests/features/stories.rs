use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::helpers::{create_user_session, spawn_test_app};

#[tokio::test]
async fn can_create_story() {
    // Arrange
    let test_app = spawn_test_app().await;
    let user_id = create_user_session("test@test.com", &test_app).await;
    let idea_id = seed_idea(&test_app.db, user_id).await;

    // Act
    let body = "story=Test%20Story";

    let url = format!("{}/{}/story", test_app.address, idea_id);
    let response = test_app
        .client
        .post(url)
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    
    // Assert
    assert!(response.status().is_success());

    assert_story_written("Test story", &test_app.db).await;

    let response_body = response.text().await.unwrap();
    assert_eq!(1, response_body.match_indices("Test Story").count());
}

#[tokio::test]
async fn can_view_stories() {
    todo!() 
}

async fn seed_idea(db: &PgPool, user_id: Uuid) -> Uuid {
    let idea_id = Uuid::new_v4();
    sqlx::query!("INSERT INTO ideas (id, title, tagline, created_at, user_id) VALUES ($1, $2, $3, $4, $5)",
        idea_id,
        "Test idea",
        "Test idea is test",
        Utc::now(),
        user_id,
    )
        .execute(db)
        .await
        .unwrap();

    idea_id
}

async fn assert_story_written(story: &str, db: &PgPool) {
    let matched_stories = sqlx::query!("SELECT COUNT(*) FROM stories WHERE story = $1", story)
        .fetch_one(db)
        .await
        .expect("Failed to find saved story");

    assert_eq!(matched_stories, 1);
}
