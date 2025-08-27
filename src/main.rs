mod components;
mod ai;
mod backend;
mod gamectrl;
mod config;

use std::time::Duration;
use dioxus::prelude::*;
use hnefatafl::preset;
use components::game::Game;
use gamectrl::GameController;
use crate::backend::new_game;
use crate::config::GameSettings;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let settings = GameSettings::new(
        preset::boards::BRANDUBH,
        preset::rules::BRANDUBH,
        Some(Duration::from_secs(5)),
        Some(Duration::from_secs(5))
    );
    let resource = use_resource(|| async move {
        new_game(&settings).await.ok()
    });
    match &*resource.read_unchecked() {
        Some(Some(game)) => rsx! {
            Game {
                controller: GameController::new(game, true, true, Duration::from_secs(6))
            }
        },
        _ => rsx! { "Loading..." },
    }
}

