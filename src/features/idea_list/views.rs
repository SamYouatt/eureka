use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        div class="h-full grid grid-cols-4 overflow-auto divide-x-2 divide-solid divide-slate-200 dark:divide-slate-700" {
            div #ideas_list class="overflow-auto divide-y divide-solid divide-slate-200 dark:divide-slate-700" {
                @for idea in ideas {
                    (idea_row(idea))
                }
            }
            div #idea_pane class="col-span-3 h-full overflow-auto" {
                div class="flex items-center justify-center h-full" {
                    p class="dark:text-white" { "What will you think of today?" }
                }
            }
        }
    }
}

fn idea_row(idea: &Idea) -> Markup {
    let route = format!("/ideas/{}", idea.id);
    let div_id = format!("idea-{}", idea.id);

    html! {
        div class="flex flex-col p-2 mx-auto hover:bg-slate-100 dark:hover:bg-indigo-800" hx-get=(route) hx-target="#idea_pane" id=(div_id) {
            h2 class="text-base text-black dark:text-white" { (idea.title) }
            p class="text-sm text-slate-500 dark:text-slate-300" { (idea.tagline) }
        }
    }
}

