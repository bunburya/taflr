use dioxus::prelude::*;
use crate::components::navbutton::NavButton;
use crate::route::Route;
use crate::components::style::CommonStyles;
use crate::message::{Message, MESSAGE};

#[component]
pub(crate) fn MainMenu() -> Element {
    rsx! {
        CommonStyles {}
        document::Stylesheet { href: asset!("/assets/css/mainmenu.css") }
        h1 { "Main Menu" }
        div {
            class: "mainmenu",
            NavButton { route: Route::NewGame, class: "mainmenu-item", text: "New Game" }
            NavButton { route: Route::LoadGame, class: "mainmenu-item", text: "Load Game" }
            NavButton { route: Route::About, class: "mainmenu-item", text: "About" }
            NavButton { route: Route::Quit, class: "mainmenu-item", text: "Quit"}
            button {
                onclick: move |_| { *MESSAGE.write() = Some(Message::error("Test error message.")); },
                "Error"
            }
            button {
                onclick: move |_| { *MESSAGE.write() = Some(Message::warning("Test warning message.")); },
                "Warning"
            }
            button {
                onclick: move |_| { *MESSAGE.write() = Some(Message::info("Test info message.")); },
                "Info"
            }
            button {
                onclick: move |_| { *MESSAGE.write() = None; },
                "Clear"
            }
        }
    }
}