use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::user::AppUser, AppState};

#[derive(serde::Deserialize)]
pub struct NewStoryForm {
    story: String,
}

pub async fn handle_create_story(
    State(state): State<AppState>,
    user: AppUser,
    Path(idea_id): Path<Uuid>,
    Form(new_story_form): Form<NewStoryForm>,
) -> impl IntoResponse {
    // Check idea is owned by user
    // Add story into table
    let new_story_id = match insert_story(&new_story_form.story, idea_id, user.id, &state.db).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    };
    // Returns story html
}

async fn insert_story(
    story: &str,
    idea_id: Uuid,
    user_id: Uuid,
    db: &PgPool,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO stories (id, story, idea_id, user_id, created_at) VALUES ($1, $2, $3, $4, $5)",
        id,
        story,
        idea_id,
        user_id,
        Utc::now(),
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert new story: {:?}", e);
        e
    })?;

    Ok(id)
}
