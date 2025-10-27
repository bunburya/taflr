use dioxus::prelude::*;
use crate::components::navbutton::NavButton;
use crate::route::Route;

const HOME: Asset = asset!("/assets/icons/home.svg");

#[component]
pub(crate) fn HeaderBar(title: String) -> Element {
    rsx! {
        div {
            class: "header-bar",
            div {
                class: "header-text",
                { title }
            }
            NavButton {
                route: Route::MainMenu,
                class: "header-nav-home",
                text: "⌂"
            }
        }
    }
}