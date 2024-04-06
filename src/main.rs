use eureka::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = run().await?;

    server.await
}
