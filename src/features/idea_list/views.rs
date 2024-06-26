use maud::{html, Markup};
use uuid::Uuid;

use super::handler::Idea;

pub fn ideas_list(ideas: &[Idea]) -> Markup {
    html! {
        div class="h-full flex flex-row gap-4 p-4 overflow-auto divide-solid divide-slate-200 dark:divide-slate-700" {
            div class="h-fit max-h-full flex flex-col shrink basis-72 max-width-72 rounded-md bg-white dark:bg-slate-800 divide-y divide-solid divide-slate-200 dark:divide-slate-700 border-2 border-slate-200 dark:border-slate-700 overflow-auto" {
                div id="ideas_list" class="overflow-auto divide-y divide-solid divide-slate-200 dark:divide-slate-700" {
                    @for idea in ideas {
                        (idea_row(&idea.title, &idea.tagline, &idea.id))
                    }
                }
                div class="p-2 bg-slate-200 dark:bg-slate-700" id="idea-list-footer" {
                    button hx-get="ideas/new/form" hx-target="#idea-list-footer" class="text-pink-500 hover:underline text-sm px-2 py-1" { "New idea" }
                }
                div id="new-idea-form-hook" class="hidden" { }
            }
            div #idea_pane class="h-full grow overflow-auto bg-white dark:bg-slate-800 rounded-md border-2 border-slate-200 dark:border-slate-700" {
                div class="flex items-center justify-center h-full" {
                    p class="dark:text-white" { "What will you think of today?" }
                }
            }
        }
    }
}

pub fn idea_row(idea_name: &str, idea_tagline: &str, idea_id: &Uuid) -> Markup {
    let route = format!("/ideas/{}", idea_id);
    let div_id = format!("idea-{}", idea_id);

    html! {
        div class="flex flex-col p-2 mx-auto hover:bg-slate-100 dark:hover:bg-slate-700" hx-get=(route) hx-target="#idea_pane" id=(div_id) {
            h2 class="text-base text-black dark:text-white" { (idea_name) }
            p class="text-xs text-slate-500 dark:text-slate-300" { (idea_tagline) }
        }
    }
}
