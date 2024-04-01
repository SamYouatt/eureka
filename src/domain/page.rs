use maud::{html, Markup, DOCTYPE};

pub fn page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { "Ideas" }
        script 
            src="https://unpkg.com/htmx.org@1.9.11"
            integrity="sha384-0gxUXCCR8yv9FM2b+U3FDbsKthCI66oH5IA9fHppQq9DDMHuMauqq1ZHBpJxQ0J0"
            crossorigin="anonymous"
            { }
        body {
            (content)
        }
    }
}
