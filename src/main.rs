mod components;
mod ai;
mod gamectrl;
mod config;
mod aictrl;
mod sqlite;
mod error;

use dioxus::prelude::*;
use crate::components::game_screen::GameScreen;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn Style() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/variables.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Stylesheet { href: asset!("/assets/css/game.css") }
        document::Stylesheet { href: asset!("/assets/css/game_setup.css") }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            Style {}
            GameScreen {}
        }
    }
}

