use dioxus::prelude::*;

#[component]
pub fn Font(cx: Scope) -> Element {
    render!(
        link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: true }
        link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Music&display=swap",
            rel: "stylesheet"
        }
    )
}
