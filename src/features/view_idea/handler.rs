use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::query_as;
use uuid::Uuid;

use super::views::{idea_view, missing_idea};
use crate::AppState;

pub struct Idea {
    pub title: String,
    pub tagline: String,
}

pub async fn get_idea(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse uuid from {:?}: {:?}", id, e);
            return missing_idea()
        },
    };

    tracing::info!("Loading idea {} from db", id);
    let query = query_as!(Idea, "SELECT title, tagline FROM ideas WHERE id = $1", id);

    let idea = match query.fetch_one(&state.db).await {
        Ok(idea) => idea,
        Err(e) => {
            println!("Failed to find idea {}: {}", id, e);
            return missing_idea();
        },
    };

    tracing::info!("Fetched idea {} successfully", id);

    idea_view(&idea)
}
