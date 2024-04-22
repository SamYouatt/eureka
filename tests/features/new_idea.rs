use crate::helpers::{create_user_session, spawn_test_app};

#[tokio::test]
async fn can_create_new_idea() {
    // Arrange
    let test_app = spawn_test_app().await;
    let user_id = create_user_session("test@test.com", &test_app).await;

    let body = "name=Test%20Idea&tagline=Just%20Testing";

    // Act
    let url = format!("{}/ideas/new", test_app.address);
    let response = test_app
        .client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let created_idea = sqlx::query!("SELECT title, tagline FROM ideas WHERE user_id = $1", user_id)
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch saved idea");

    assert_eq!(created_idea.title, "Test Idea");
    assert_eq!(created_idea.tagline, "Just Testing");
}
