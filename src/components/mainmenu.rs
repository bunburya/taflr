use dioxus::prelude::*;
use crate::route::Route;
use crate::components::style::CommonStyles;

#[component]
fn NavButton(route: Route, text: &'static str) -> Element {
    rsx! {
        button {
            class: "mainmenu-item",
            onclick: move |_| {
                let nav = navigator();
                nav.push(route);
            },
            { text }
        }
    }
}

#[component]
pub(crate) fn MainMenu() -> Element {
    rsx! {
        CommonStyles {}
        document::Stylesheet { href: asset!("/assets/css/mainmenu.css") }
        h1 { "Main Menu" }
        div {
            class: "mainmenu",
            NavButton { route: Route::NewGame, text: "New Game" }
            NavButton { route: Route::LoadGame, text: "Load Game" }
            NavButton { route: Route::About, text: "About" }
        }
    }
}