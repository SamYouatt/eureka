use axum::{extract::State, response::IntoResponse};

use crate::{domain::page::page, AppState};

use super::views::ideas_list;


pub async fn get_ideas(State(state): State<AppState>) -> impl IntoResponse {
    let ideas = state.ideas.lock().unwrap().to_vec();

    page(ideas_list(&ideas))
}
