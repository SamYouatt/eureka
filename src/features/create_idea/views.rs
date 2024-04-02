use maud::{html, Markup};

pub fn new_idea_form() -> Markup {
    html! {
        form hx-post="/ideas/new" {
            label for="title" { "Name:" }
            input id="name" name="name" type="text" placeholder="Start candle business" { }
            label for="tagline" { "Tagline:" }
            input id="tagline" name="tagline" type="text" placeholder="Cheaper than electric" { }
            button { "Create" }
        }
    }
}
