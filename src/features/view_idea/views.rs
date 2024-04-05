use maud::{html, Markup};

use crate::domain::idea::Idea;

pub fn idea_pane_contents(idea: &Idea) -> Markup {
    html! {
        p { (idea.title) }
        p { (idea.tagline) }
    }
}

pub fn missing_idea() -> Markup {
    html! {
        p { "Oops! Something went wrong finding your idea..." }
    }
}
