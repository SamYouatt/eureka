use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;

use crate::{AppState, Idea};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    ideas: Vec<Idea>
}

pub async fn get_ideas(State(state): State<AppState>) -> impl IntoResponse {
    let ideas = state.ideas.lock().unwrap().to_vec();

    IndexTemplate { ideas }
}
