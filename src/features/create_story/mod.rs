use axum::{extract::Path, response::IntoResponse};
use uuid::Uuid;

pub async fn handle_create_story(Path(idea_id): Path<Uuid>) -> impl IntoResponse {
    todo!()
}
