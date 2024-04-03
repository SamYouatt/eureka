use axum::{extract::State, response::IntoResponse};
use maud::{html, Markup};

use crate::{domain::page::page, AppState, Idea};

fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        a href="/ideas/new" { "New idea" }
        div #ideas_list class="text-indigo-500" {
            @for idea in ideas {
                (idea.card_markup())
            }
        }
    }
}

pub async fn get_ideas(State(state): State<AppState>) -> impl IntoResponse {
    let ideas = state.ideas.lock().unwrap().to_vec();

    page(ideas_list(&ideas))
}
