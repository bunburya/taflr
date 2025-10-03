use dioxus::prelude::*;

/// Stylesheets that are used across different pages.
#[component]
pub(crate) fn CommonStyles() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/variables.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
    }
}