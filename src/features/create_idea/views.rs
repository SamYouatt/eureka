use maud::{html, Markup};

pub fn new_idea_form() -> Markup {
    html! {
        form hx-post="/ideas/new" id="new-idea-form" {
            label for="title" { "Name:" }
            input id="name" name="name" type="text" placeholder="Start candle business" { }
            label for="tagline" { "Tagline:" }
            input id="tagline" name="tagline" type="text" placeholder="Cheaper than electric" { }
            button hx-post="/ideas/new/cancel" hx-target="#idea-list-footer" { "Cancel" }
            button type="submit" { "Create" }
        }
    }
}

pub fn new_idea_button() -> Markup {
    html! {
        button hx-get="ideas/new/form" hx-target="#idea-list-footer" { "New idea" }
    }
}
