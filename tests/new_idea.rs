use crate::startup::spawn_test_app;

pub mod startup;

#[tokio::test]
async fn can_creat_new_idea() {
    // Arrange
    let test_app = spawn_test_app().await;

    let client = reqwest::Client::new();
    let url = format!("{}/ideas/new", test_app.address);

    let body = "name=Test%20Idea&tagline=Just%20Testing";

    // Act
    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let created_idea = sqlx::query!("SELECT title, tagline FROM ideas")
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch saved idea");

    assert_eq!(created_idea.title, "Test Idea");
    assert_eq!(created_idea.tagline, "Just Testing");
}
