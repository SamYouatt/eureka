use eureka::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:42069").await.expect("Failed to bind listener");
    let server = run(listener).await?;

    server.await
}
