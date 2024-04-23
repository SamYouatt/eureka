use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::helpers::{create_user_session, seed_user, spawn_test_app};

#[tokio::test]
async fn can_view_idea_list() {
    // Arrange
    let test_app = spawn_test_app().await;
    let user_id = create_user_session("test@test.com", &test_app).await;
    seed_ideas(&test_app.db, user_id).await;

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

#[tokio::test]
async fn can_only_view_own_idea_list() {
    // Arrange
    let test_app = spawn_test_app().await;
    let alice = Uuid::new_v4();
    seed_user(alice, "alice@test.com", &test_app.db).await;
    seed_ideas(&test_app.db, alice).await;

    let bob_user_id = create_user_session("bob@test.com", &test_app).await;
    seed_ideas(&test_app.db, bob_user_id).await;

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

async fn seed_ideas(db: &PgPool, user_id: Uuid) {
    let idea1 = Idea {
        title: "First idea".into(),
        tagline: "One".into(),
    };
    insert_idea(db, idea1, user_id).await;
    let idea2 = Idea {
        title: "Second idea".into(),
        tagline: "Two".into(),
    };
    insert_idea(db, idea2, user_id).await;
}

async fn insert_idea(db: &PgPool, idea: Idea, user_id: Uuid) {
    sqlx::query!(
        "INSERT INTO ideas (id, title, tagline, created_at, user_id) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4(),
        idea.title,
        idea.tagline,
        Utc::now(),
        user_id,
    )
    .execute(db)
    .await
    .unwrap();
}

struct Idea {
    title: String,
    tagline: String,
}
