use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        a href="/ideas/new" { "New idea" }
        div #ideas_list {
            @for idea in ideas {
                (idea_card(idea))
            }
        }
    }
}

fn idea_card(idea: &Idea) -> Markup {
    html! {
        div class="flex flex-col p-4 max-w-sm mx-auto rounded-lg shadow-lg space-y-2" {
            h2 class="text-xl text-black" { (idea.title) }
            p class="text-slate-500" { (idea.tagline) }
        }
    }
}
