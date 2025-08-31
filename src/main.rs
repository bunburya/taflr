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
fn App() -> Element {
    rsx! {
        GameScreen {}
    }
}

