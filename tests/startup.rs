use eureka::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use tokio::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
}

pub async fn spawn_test_app() -> TestApp {
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(listener, db_pool.clone()).await.expect("Failed to spawn server");

    let _ = tokio::spawn(async { server.await });

    TestApp {
        address,
        db: db_pool,
    }
}
