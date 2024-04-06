use eureka::{configuration::get_configuration, startup::run};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let app_address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(app_address).await.expect("Failed to bind listener");

    let server = run(listener).await?;
    server.await
}
