use eureka::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Tracing setup
    let subscriber = get_subscriber("eureka".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // DB setup
    let db_pool = PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");

    // Network setup
    let app_address = format!("{}:{}", configuration.application.host, configuration.application.port);
    
    let listener = TcpListener::bind(app_address)
        .await
        .expect("Failed to bind listener");

    let server = run(listener, db_pool).await?;
    server.await
}
