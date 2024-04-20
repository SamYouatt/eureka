use axum_extra::extract::cookie::Key;
use eureka::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("eureka".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database.with_db());

    let open_id_client = configuration.openid.build_client();

    let app_address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let http_client = Client::new();
    let cookie_signing_key = Key::generate();

    let listener = TcpListener::bind(app_address)
        .await
        .expect("Failed to bind listener");

    let server = run(listener, db_pool, open_id_client, http_client, cookie_signing_key).await?;
    server.await
}
