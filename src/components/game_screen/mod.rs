mod board;
mod piece;
mod icons;
mod square;
mod game;
mod info_panel;
pub mod setup;

use std::ops::Deref;
use dioxus::core_macro::rsx;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use hnefatafl::game::MediumBasicGame;
use crate::backend::get_game_and_settings;
use crate::components::game_screen::game::Game;
use crate::components::GameSetupScreen;
use crate::config::GameSettings;
use crate::gamectrl::GameController;

static GAME_SETTINGS: GlobalSignal<Option<(MediumBasicGame, GameSettings)>> = Signal::global(|| None);

#[component]
pub(crate) fn GameScreen() -> Element {

    match GAME_SETTINGS.read().deref() {
        Some((game, settings)) => rsx! {
            Game { game_ctrl: GameController::new(
                game,
                settings.attacker.clone(),
                settings.defender.clone(),
                settings.variant_name.clone(),
            )},
        },
        None => rsx! {
            GameSetupScreen {}
        },
        _ => rsx! { "Loading..." },
    }
}