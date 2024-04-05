use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn idea_view(idea: &Idea) -> Markup {
    html! {
        div class="flex flex-col h-full overflow-auto p-4 gap-4" {
            h1 class="text-3xl dark:text-white" { (idea.title) }
            h2 class="text-lg text-indigo-500" { (idea.tagline) }
        }
    }
}

pub fn missing_idea() -> Markup {
    html! {
        div class="flex items-center justify-center h-full" {
            p class="dark:text-white" { "Oops! Something went wrong finding your idea..." }
        }
    }
}
