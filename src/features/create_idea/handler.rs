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

use crate::{domain::page::page, AppState, Idea};

use super::views::new_idea_form;

pub async fn create_idea_page() -> impl IntoResponse {
    page(new_idea_form())
}

#[derive(Deserialize)]
pub struct NewIdea {
    name: String,
    tagline: String,
}

pub async fn create_idea(
    State(state): State<AppState>,
    Form(new_idea): Form<NewIdea>,
) -> impl IntoResponse {
    let new_idea = Idea::new(&new_idea.name, &new_idea.tagline);

    if let Err(e) = query!(
        "INSERT INTO ideas (id, title, tagline, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        new_idea.title,
        new_idea.tagline,
        Utc::now()
    )
    .execute(&state.db)
    .await
    {
        println!("Failed to execute query: {}", e);
        return (HeaderMap::new(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", "/".parse().unwrap());

    (headers, StatusCode::OK)
}
