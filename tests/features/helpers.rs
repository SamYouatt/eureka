use eureka::{
    configuration::{get_configuration, DatabaseSettings},
    startup::{get_db_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
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
});

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
    pub open_id_client: MockServer,
}

pub async fn spawn_test_app() -> TestApp {
    Lazy::force(&TRACING);

    let open_id_client = MockServer::start().await;

    // Randomise config for each test so they can be isolated
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.openid.auth_url = format!("{}/auth", open_id_client.uri());
        c.openid.token_url = format!("{}/token", open_id_client.uri());
        c.openid.user_info_url = format!("{}/user", open_id_client.uri());
        c
    };

    configure_and_migrate_db(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build appliaction");
    let address = format!("http:127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db: get_db_pool(&configuration.database),
        open_id_client,
    }
}

// Create a new database instance for each test
async fn configure_and_migrate_db(config: &DatabaseSettings) -> PgPool {
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

pub fn assert_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}
