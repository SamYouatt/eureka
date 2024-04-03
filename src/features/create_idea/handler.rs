use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Form,
};
use serde::Deserialize;

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

    state.ideas.lock().unwrap().push(new_idea.clone());

    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", "/".parse().unwrap());

    (headers, StatusCode::OK)
}
