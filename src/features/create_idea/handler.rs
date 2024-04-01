use axum::extract::State;
use maud::Markup;

use crate::{AppState, Idea};


pub async fn create_idea(State(state): State<AppState>) -> Markup {
    let new_idea = Idea::new("Random", "cool");

    state.ideas.lock().unwrap().push(new_idea.clone());

    new_idea.card_markup()
}
