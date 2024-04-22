use eureka::{
    configuration::{get_configuration, DatabaseSettings},
    startup::{get_db_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::{matchers::{method, path}, Mock, MockGuard, MockServer, ResponseTemplate};

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
    let address = format!("http://{}:{}", configuration.application.domain, application.port());
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

#[derive(serde::Serialize)]
struct UseInfoResponse {
    email: String,
}

#[derive(serde::Serialize)]
struct AuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl AuthTokenResponse {
    fn new() -> Self {
        Self {
            access_token: "test_access_token".into(),
            token_type: "basic".into(),
            expires_in: 3600,
        }
    }
}

pub async fn configure_open_id_mock(test_app: &TestApp) -> (MockGuard, MockGuard) {
    // These routes are mocked and not the real routes used by Google
    let auth_token_response = ResponseTemplate::new(200).set_body_json(AuthTokenResponse::new());
    let mock_token_exchange = Mock::given(path("/token"))
        .and(method("POST"))
        .respond_with(auth_token_response)
        .named("OpenId auth token exchange")
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    let user_info_response = ResponseTemplate::new(200).set_body_json(UseInfoResponse {
        email: "test@test.com".into(),
    });
    let mock_user_info = Mock::given(path("/user"))
        .and(method("GET"))
        .respond_with(user_info_response)
        .named("OpenId user info")
        .expect(1)
        .mount_as_scoped(&test_app.open_id_client)
        .await;

    (mock_token_exchange, mock_user_info)
}

// Creates a loggen in user with session cookie
pub async fn run_login(client: &reqwest::Client, test_app: &TestApp) {
    let _mock_open_id = configure_open_id_mock(test_app).await;

    let url = format!("{}/login/redirect?code=testauthcode", test_app.address);
    let _response = &client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");
}

pub fn assert_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}
