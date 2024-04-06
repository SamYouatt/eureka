use eureka::run;

#[tokio::test]
async fn server_is_launched() {
    // Arrange
    spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:42069/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() {
    let server = run().await.expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });
}
