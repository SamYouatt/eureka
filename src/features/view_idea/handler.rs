use axum::{extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;

use super::views::{idea_view, missing_idea};
use crate::AppState;

pub async fn get_idea(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return missing_idea(),
    };
    
    match state.ideas.lock().unwrap().iter().find(|idea| idea.id == id) {
        Some(idea) => idea_view(&idea),
        None => missing_idea(),
    }
}
