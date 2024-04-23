use crate::helpers::spawn_test_app;

#[tokio::test]
async fn server_is_launched() {
    // Arrange
    let test_app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/health_check", test_app.address);

    // Act
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
