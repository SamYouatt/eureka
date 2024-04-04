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
        link href="/assets/main.css" rel="stylesheet";
        link href="https://rsms.me/inter/inter.css" rel="stylesheet";
        body {
            (navbar())
            (content)
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav class="bg-gray-900 w-full z-20 border-b border-gray-600 flex flex-wrap items-center justify-between mx-auto p-4" {
            a href="/" class="text-3xl text-indigo-500 font-bold" { "Eureka" }
            a
                href="/ideas/new"
                class="bg-indigo-500 text-white hover:bg-indigo-700 focus:ring-4 focus:outline-none rounded-lg text-sm px-4 py-2 text-center"
                { "New Idea" }
        }
    }
}
