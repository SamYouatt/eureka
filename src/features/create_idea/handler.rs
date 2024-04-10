use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

use crate::{domain::page::page, AppState};

use super::views::{new_idea_button, new_idea_form};

#[tracing::instrument(name = "Render new idea form")]
pub async fn create_idea_page() -> impl IntoResponse {
    page(new_idea_form())
}

pub async fn get_idea_form() -> impl IntoResponse {
    new_idea_form()
}

pub async fn cancel_idea_form() -> impl IntoResponse {
    new_idea_button()
}

#[derive(Deserialize, Debug)]
pub struct NewIdea {
    name: String,
    tagline: String,
}

#[tracing::instrument(
    name="Creating new idea",
    skip(state, new_idea),
    fields(
        idea_title = %new_idea.name,
    )
)]
pub async fn create_idea(
    State(state): State<AppState>,
    Form(new_idea): Form<NewIdea>,
) -> impl IntoResponse {
    match insert_idea(&state.db, new_idea).await {
        Ok(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("HX-Redirect", "/".parse().unwrap());

            (headers, StatusCode::OK)
        }
        Err(_) => (HeaderMap::new(), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tracing::instrument(name = "Saving new idea to database", skip(db, idea))]
async fn insert_idea(db: &PgPool, idea: NewIdea) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO ideas (id, title, tagline, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        idea.name,
        idea.tagline,
        Utc::now()
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error! {"Failed to execute query: {:?}", e};
        e
    })?;

    Ok(())
}
