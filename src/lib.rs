use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub async fn run() {
    let ideas = Arc::new(Mutex::new(generate_seed_data()));

    let state = AppState {
        ideas: ideas.clone(),
    };

    let assets_path = std::env::current_dir().unwrap();

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/health_check", get(health_check))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .route("/ideas/:id", get(get_idea))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
