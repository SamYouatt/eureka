use eureka::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Tracing setup
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("eureka".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set tracing subscriber");

    // Load configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // DB setup
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Network setup
    let app_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(app_address)
        .await
        .expect("Failed to bind listener");

    let server = run(listener, db_pool).await?;
    server.await
}
