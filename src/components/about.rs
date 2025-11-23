use dioxus::prelude::*;
use crate::components::header_bar::HeaderBar;

#[component]
pub(crate) fn About() -> Element {
    rsx! {
        div {
            class: "main-container",
            HeaderBar {
                title: "About",
            }
            div {
                class: "about-container",
                "This is the about page"
            }
        }
    }
}