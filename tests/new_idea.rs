use eureka::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};
use tokio::net::TcpListener;

#[tokio::test]
async fn can_creat_new_idea() {
    // Arrange
    let app_address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let url = format!("{}/ideas/new", app_address);
    
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
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved idea");

    assert_eq!(created_idea.title, "Test Idea");
    assert_eq!(created_idea.tagline, "Just Testing");
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).await.expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });

    format!("http://127.0.0.1:{}", port) 
}
