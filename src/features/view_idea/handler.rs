use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use super::views::{idea_view, missing_idea};
use crate::{domain::user::AppUser, AppState};

enum CreateIdeaError {
    SqlError,
    Unauthorised,
}

impl IntoResponse for CreateIdeaError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateIdeaError::SqlError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            CreateIdeaError::Unauthorised => StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}

pub struct Idea {
    pub title: String,
    pub tagline: String,
}

#[tracing::instrument(name = "Get idea", skip(state))]
pub async fn get_idea(State(state): State<AppState>, user: AppUser, Path(id): Path<String>) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse uuid from {:?}: {:?}", id, e);
            return missing_idea().into_response();
        }
    };

    match fetch_idea(id, &user, &state.db).await {
        Ok(idea) => idea_view(&idea).into_response(),
        Err(e) => e.into_response(),
    }
}

#[tracing::instrument(name = "Fetch idea from database", skip(db))]
async fn fetch_idea(id: Uuid, user: &AppUser, db: &PgPool) -> Result<Idea, CreateIdeaError> {
    query_as!(Idea, "SELECT title, tagline FROM ideas WHERE id = $1 AND user_id = $2", id, user.id)
        .fetch_one(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => CreateIdeaError::Unauthorised,
                _ => CreateIdeaError::SqlError,
            }
        })
}
