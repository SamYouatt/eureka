use maud::{html, Markup};

pub fn new_idea_form() -> Markup {
    html! {
        form hx-post="/ideas/new" id="new-idea-form" class="flex flex-col" hx-target="this" hx-swap="outerHtml" {
            div class="flex flex-col gap-1" {
                label for="title" class="text-sm text-slate-600 dark:text-slate-200" { "Name:" }
                input id="name" name="name" type="text" required minlength="3" maxlength="32" placeholder="Start candle business" class ="px-1 rounded-sm dark:bg-slate-800" { }
            }
            div class="flex flex-col gap-1 mt-2" {
                label for="tagline" class="text-sm text-slate-600 dark:text-slate-200" { "Tagline:" }
                input id="tagline" name="tagline" type="text" required placeholder="Cheaper than electric" class ="px-1 rounded-sm dark:bg-slate-800" { }
            }
            div class="flex gap-1 mt-4" {
                button type="submit" class="bg-pink-500 text-white hover:bg-pink-700 text-sm rounded-md px-2 py-1 text-center" { "Create" }
                button hx-post="/ideas/new/cancel" hx-target="#idea-list-footer" class="text-pink-500 hover:underline text-sm px-2 py-1" { "Cancel" }
            }
        }
    }
}

pub fn error_form(current_name: &str, current_tagline: &str, name_error: &str) -> Markup {
    html! {
        form hx-post="/ideas/new" id="new-idea-form" class="flex flex-col" {
            div class="flex flex-col gap-1" {
                label for="title" class="text-sm text-slate-600 dark:text-slate-200" { "Name:" }
                input id="name" name="name" type="text" required minlength="3" maxlength="32" placeholder="Start candle business" value=(current_name) class ="px-1 rounded-sm border-red-500 border-2 dark:bg-slate-800" {}
                p class="text-sm text-red-500 italic" { (name_error) }
            }
            div class="flex flex-col gap-1 mt-2" {
                label for="tagline" class="text-sm text-slate-600 dark:text-slate-200" { "Tagline:" }
                input id="tagline" name="tagline" type="text" placeholder="Cheaper than electric" value=(current_tagline) class ="px-1 rounded-sm dark:bg-slate-800" {}
            }
            div class="flex gap-1 mt-4" {
                button type="submit" class="bg-pink-500 text-white hover:bg-pink-700 text-sm rounded-md px-2 py-1 text-center" { "Create" }
                button hx-post="/ideas/new/cancel" hx-target="#idea-list-footer" class="text-pink-500 hover:underline text-sm px-2 py-1" { "Cancel" }
            }
        }
    }
}

pub fn new_idea_button() -> Markup {
    html! {
        button hx-get="ideas/new/form" hx-target="#idea-list-footer" class="text-pink-500 hover:underline text-sm px-2 py-1" { "New idea" }
    }
}
