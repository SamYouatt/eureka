use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use super::views::{idea_view, missing_idea};
use crate::AppState;

pub struct Idea {
    pub title: String,
    pub tagline: String,
}

#[tracing::instrument(name = "Get idea", skip(state))]
pub async fn get_idea(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse uuid from {:?}: {:?}", id, e);
            return missing_idea();
        }
    };

    match fetch_idea(id, &state.db).await {
        Ok(idea) => idea_view(&idea),
        Err(_) => missing_idea(),
    }
}

#[tracing::instrument(name = "Fetch idea from database", skip(db))]
async fn fetch_idea(id: Uuid, db: &PgPool) -> Result<Idea, sqlx::Error> {
    query_as!(Idea, "SELECT title, tagline FROM ideas WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })
}
