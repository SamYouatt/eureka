use axum::{extract::Path, response::IntoResponse, Form};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewStoryForm {
    story: String,
}

pub async fn handle_create_story(Path(idea_id): Path<Uuid>, Form(new_story_form): Form<NewStoryForm>) -> impl IntoResponse {
    todo!()
}
