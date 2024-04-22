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

use crate::{domain::{page::page, user::AppUser}, AppState};

use super::views::{error_form, new_idea_button, new_idea_form, new_idea_row};

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

#[derive(Deserialize, Debug, Clone)]
pub struct NewIdeaForm {
    name: String,
    tagline: String,
}

pub struct NewIdea {
    pub name: String,
    pub tagline: String,
}

impl TryFrom<NewIdeaForm> for NewIdea {
    type Error = String;

    fn try_from(value: NewIdeaForm) -> Result<Self, Self::Error> {
        if value.name.len() < 3 {
            return Err("Your idea name must be at least 3 characters long".into());
        }

        if value.name.len() > 64 {
            return Err("Your idea name can't be longer than 64 characters".into());
        }

        Ok(NewIdea {
            name: value.name,
            tagline: value.tagline,
        })
    }
}

#[tracing::instrument(
    name="Creating new idea",
    skip(state, new_idea_form),
    fields(
        idea_title = %new_idea_form.name,
    )
)]
pub async fn create_idea(
    State(state): State<AppState>,
    user: AppUser,
    Form(new_idea_form): Form<NewIdeaForm>,
) -> axum::response::Response {
    let new_idea = match NewIdea::try_from(new_idea_form.clone()) {
        Ok(new_idea) => new_idea,
        Err(new_idea_error) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                error_form(&new_idea_form.name, &new_idea_form.tagline, &new_idea_error),
            )
                .into_response()
        }
    };

    match insert_idea(&state.db, &new_idea, &user).await {
        Ok(idea_id) => {
            return new_idea_row(&new_idea, idea_id).into_response();
        }
        Err(_) => (HeaderMap::new(), StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[tracing::instrument(name = "Saving new idea to database", skip(db, idea))]
async fn insert_idea(db: &PgPool, idea: &NewIdea, user: &AppUser) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();

    query!(
        "INSERT INTO ideas (id, title, tagline, created_at, user_id) VALUES ($1, $2, $3, $4, $5)",
        id,
        idea.name,
        idea.tagline,
        Utc::now(),
        user.id,
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error! {"Failed to execute query: {:?}", e};
        e
    })?;

    Ok(id)
}
