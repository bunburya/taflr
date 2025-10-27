use dioxus::prelude::*;
use crate::components::header_bar::HeaderBar;

#[component]
pub(crate) fn About() -> Element {
    rsx! {
        HeaderBar {
            title: "About",
        }
        "This is the about page"
    }
}