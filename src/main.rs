use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use domain::idea::Idea;
use features::{
    create_idea::handler::{create_idea, create_idea_page},
    idea_list::handler::get_ideas,
};
use tower_http::services::ServeDir;

mod domain;
mod features;

#[derive(Clone)]
pub struct AppState {
    ideas: Arc<Mutex<Vec<Idea>>>,
}

#[tokio::main]
async fn main() {
    let seed_idea = Idea::new("First idea", "bosh");

    let ideas = Arc::new(Mutex::new(vec![]));
    ideas.lock().unwrap().push(seed_idea);

    let state = AppState {
        ideas: ideas.clone(),
    };

    let assets_path = std::env::current_dir().unwrap();

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
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
