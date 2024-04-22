use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::helpers::{run_login, spawn_test_app};

#[tokio::test]
async fn can_view_idea_list() {
    // Arrange
    let test_app = spawn_test_app().await;
    run_login(&test_app).await;
    seed_ideas(&test_app.db).await;

    // Act
    let url = format!("{}/", test_app.address);
    let response = test_app
        .client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let response_body = response.text().await.unwrap();
    assert_eq!(1, response_body.match_indices("First idea").count());
    assert_eq!(1, response_body.match_indices("Second idea").count());
}

async fn seed_ideas(db: &PgPool) {
    let idea1 = Idea {
        title: "First idea".into(),
        tagline: "One".into(),
    };
    insert_idea(db, idea1).await;
    let idea2 = Idea {
        title: "Second idea".into(),
        tagline: "Two".into(),
    };
    insert_idea(db, idea2).await;
}

async fn insert_idea(db: &PgPool, idea: Idea) {
    sqlx::query!(
        "INSERT INTO ideas (id, title, tagline, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        idea.title,
        idea.tagline,
        Utc::now()
    )
    .execute(db)
    .await
    .unwrap();
}

struct Idea {
    title: String,
    tagline: String,
}
