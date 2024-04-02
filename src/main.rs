use std::sync::{Arc, Mutex};

use axum::{
    routing::get,
    Router,
};
use domain::idea::Idea;
use features::{create_idea::handler::{create_idea, create_idea_page}, idea_list::handler::get_ideas};

mod features;
mod domain;

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

    let app = Router::new()
        .route("/", get(get_ideas))
        .route("/ideas/new", get(create_idea_page).post(create_idea))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

