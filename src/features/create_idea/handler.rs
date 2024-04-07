use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

use crate::{domain::page::page, AppState};

use super::views::new_idea_form;

pub async fn create_idea_page() -> impl IntoResponse {
    page(new_idea_form())
}

#[derive(Deserialize, Debug)]
pub struct NewIdea {
    name: String,
    tagline: String,
}

pub async fn create_idea(
    State(state): State<AppState>,
    Form(new_idea): Form<NewIdea>,
) -> impl IntoResponse {
    tracing::info!("Inserting new idea into database: {:?}", new_idea);
    if let Err(e) = query!(
        "INSERT INTO ideas (id, title, tagline, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        new_idea.name,
        new_idea.tagline,
        Utc::now()
    )
    .execute(&state.db)
    .await
    {
        tracing::error!("Failed to execute query: {:?}", e);
        return (HeaderMap::new(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Idea inserted succesfully");

    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", "/".parse().unwrap());

    (headers, StatusCode::OK)
}
