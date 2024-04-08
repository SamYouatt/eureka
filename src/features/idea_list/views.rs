use maud::{html, Markup};

use super::handler::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        div class="h-full flex flex-row gap-4 p-4 overflow-auto divide-solid divide-slate-200 dark:divide-slate-700" {
            div #ideas_list class="rounded-md bg-white h-fit shrink basis-72 max-width-72 divide-y divide-solid divide-slate-200" {
                div class="overflow-auto divide-y divide-solid divide-slate-200 dark:divide-slate-700" {
                    @for idea in ideas {
                        (idea_row(idea))
                    }
                }
                div class="flex flex-row justify-start align-middle px-2 py-2" {
                    a href="/ideas/new" class="bg-pink-500 text-white hover:bg-pink-700 text-sm rounded-md px-2 py-1 text-center" { "New idea" }    
                }
            }
            div #idea_pane class="h-full grow overflow-auto bg-white rounded-md" {
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
            p class="text-xs text-slate-500 dark:text-slate-300" { (idea.tagline) }
        }
    }
}
