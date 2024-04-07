use axum::{extract::State, response::IntoResponse};
use maud::html;
use sqlx::query_as;
use uuid::Uuid;

use crate::{domain::page::page, AppState};

use super::views::ideas_list;

pub struct Idea {
    pub id: Uuid,
    pub title: String,
    pub tagline: String,
}

pub async fn get_ideas(State(state): State<AppState>) -> impl IntoResponse {
    let ideas: Vec<Idea> = match query_as!(Idea, "SELECT id, title, tagline FROM ideas")
        .fetch_all(&state.db)
        .await
    {
        Ok(ideas) => ideas,
        Err(e) => {
            println!("Failed to get ideas from db: {e}");
            return html!{ p { "Oops, something went wrong getting your ideas..." } };
        },
    };

    page(ideas_list(&ideas))
}
