use eureka::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Tracing setup
    let subscriber = get_subscriber("eureka".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // DB setup
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database.with_db());

    // Auth setup
    let open_id_client = configuration.openid.build_client();

    // Network setup
    let app_address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(app_address)
        .await
        .expect("Failed to bind listener");

    let server = run(listener, db_pool, open_id_client).await?;
    server.await
}
