use axum::{extract::State, response::IntoResponse};
use maud::html;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::{
    domain::{page::page, user::AppUser},
    AppState,
};

use super::views::ideas_list;

pub struct Idea {
    pub id: Uuid,
    pub title: String,
    pub tagline: String,
}

#[tracing::instrument(name = "Get ideas list", skip(state))]
pub async fn get_ideas(State(state): State<AppState>, user: AppUser) -> impl IntoResponse {
    match fetch_ideas(&user, &state.db).await {
        Ok(ideas) => page(ideas_list(&ideas)),
        Err(_) => page(html! { p { "Oops, something went wrong getting your ideas..." } }),
    }
}

#[tracing::instrument(name = "Fetch ideas from database", skip(db))]
async fn fetch_ideas(user: &AppUser, db: &PgPool) -> Result<Vec<Idea>, sqlx::Error> {
    query_as!(
        Idea,
        "SELECT id, title, tagline FROM ideas WHERE user_id = $1",
        user.id
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        tracing::error! {"Failted to execute query: {:?}", e};
        e
    })
}
