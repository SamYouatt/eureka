use axum_extra::extract::cookie::Key;
use eureka::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
}

pub async fn spawn_test_app() -> TestApp {
    configure_subscriber();

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database).await;

    let open_id_client = configuration.openid.build_client();

    let http_client = Client::new();
    let cookie_signing_key = Key::generate();

    let server = run(listener, db_pool.clone(), open_id_client, http_client, cookie_signing_key)
        .await
        .expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });

    TestApp {
        address,
        db: db_pool,
    }
}

fn configure_subscriber() {
    let subscriber_name = "test".to_string();
    let default_filter_level = "info".to_string();

    // Tracing should match behaviour of prints in tests, and by default print to void unless asked
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
}

// Create a new database instance for each test
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    db_pool
}
