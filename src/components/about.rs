use dioxus::prelude::*;

#[component]
pub(crate) fn About() -> Element {
    rsx! {
        h1 { "About" }
        "This is the about page"
    }
}