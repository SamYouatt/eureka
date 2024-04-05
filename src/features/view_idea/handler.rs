use axum::{extract::Path, response::IntoResponse};
use maud::html;
use uuid::Uuid;

// /idea/{id}
pub async fn get_idea(Path(id): Path<String>) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return html!{ p { "Couldn't find matching idea" } },
    };

    html! {
        p { (id) }
    }
}
