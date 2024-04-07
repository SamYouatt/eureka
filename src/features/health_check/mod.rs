use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    tracing::info!("Responding to health check, I'm alive!");

    StatusCode::OK
}
