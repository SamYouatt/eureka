use askama_axum::IntoResponse;
use axum::extract::State;

use crate::{AppState, Idea, IdeaCard};

pub async fn create_idea(State(state): State<AppState>) -> impl IntoResponse {
    let new_idea = Idea::new("Random", "cool");

    state.ideas.lock().unwrap().push(new_idea.clone());

    IdeaCard::from_idea(new_idea)
}
