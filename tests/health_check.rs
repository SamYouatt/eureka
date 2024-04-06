use eureka::run;
use tokio::net::TcpListener;

#[tokio::test]
async fn server_is_launched() {
    // Arrange
    let port = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/health_check", port);

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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).await.expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });

    format!("http://127.0.0.1:{}", port) 
}
