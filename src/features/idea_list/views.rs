use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        div class="h-full grid grid-cols-3 overflow-auto divide-x-2 divide-solid divide-gray-200" {
            div #ideas_list class="overflow-auto divide-y divide-solid divide-gray-200" {
                @for idea in ideas {
                    (idea_card(idea))
                }
            }
            div #idea_pane class="col-span-2 flex justify-center items-center" {
                p { "Idea here" }
            }
        }
    }
}

fn idea_card(idea: &Idea) -> Markup {
    let route = format!("/ideas/{}", idea.id);
    let div_id = format!("idea-{}", idea.id);

    html! {
        div class="flex flex-col p-2 mx-auto hover:bg-indigo-100" hx-get=(route) hx-target="#idea_pane" id=(div_id) {
            h2 class="text-base text-black" { (idea.title) }
            p class="text-sm text-slate-500" { (idea.tagline) }
        }
    }
}

