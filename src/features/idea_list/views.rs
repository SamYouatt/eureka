use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        div class="grid grid-cols-3 overflow-auto" {
            div #ideas_list class="overflow-auto" {
                @for idea in ideas {
                    (idea_card(idea))
                }
            }
            div class="col-span-2 flex justify-center items-center" {
                p { "Idea here" }
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
