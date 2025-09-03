mod components;
mod ai;
mod backend;
mod gamectrl;
mod config;

use dioxus::prelude::*;
use crate::components::game_screen::GameScreen;
use crate::components::GameSetupScreen;

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
        document::Stylesheet { href: asset!("/assets/css/board.css") }
        document::Stylesheet { href: asset!("/assets/css/game_setup.css") }
        document::Stylesheet { href: asset!("/assets/css/game_info_panel.css") }
        document::Stylesheet { href: asset!("/assets/css/game_side_pane.css") }
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

