use eureka::startup::run;
use tokio::net::TcpListener;

#[tokio::test]
async fn can_creat_new_idea() {
    // Arrange
    let port = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/ideas/new", port);
    
    let body = "";

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
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).await.expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });

    format!("http://127.0.0.1:{}", port) 
}
