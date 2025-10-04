use dioxus::prelude::*;
use crate::route::Route;

#[component]
pub(crate) fn NavButton(
    #[props(default = false)] replace: bool,
    #[props(default = "")] mut class: &'static str,
    route: Route,
    text: &'static str
) -> Element {
    let cls = if class.is_empty() {
        "nav-button".to_string()
    } else {
        [class, "nav-button"].join(" ")
    };
    rsx! {
        Link {
            class: cls,
            to: route,
            { text }
        }
    }
}
