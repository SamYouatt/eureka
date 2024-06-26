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
            {}
        script src="/assets/main.js" {}
        link href="/assets/main.css" rel="stylesheet";
        link href="https://rsms.me/inter/inter.css" rel="stylesheet";
        body class="h-dvh flex flex-col bg-slate-100 dark:bg-slate-900 dark:[color-scheme:dark]" {
            (navbar())
            (content)
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav class="bg-white dark:bg-slate-800 w-full flex flex-wrap items-center justify-between mx-auto p-4 border-b-2 border-slate-200 dark:border-slate-700" {
            a href="/" class="text-3xl text-pink-500 font-bold" { "Eureka" }
        }
    }
}
