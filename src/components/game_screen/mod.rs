mod board;
mod piece;
mod icons;
mod square;
mod game;
mod info_panel;
pub mod setup;
mod ctrl_panel;
mod side_pane;

use std::collections::HashSet;
use std::ops::Deref;
use std::time::Instant;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use hnefatafl::game::MediumBasicGame;
use crate::components::game_screen::game::Game;
use crate::components::GameSetupScreen;
use crate::config::GameSettings;
use crate::gamectrl::{GameController, Player};

static GAME_CTRL: GlobalSignal<Option<(MediumBasicGame, GameSettings)>> = Signal::global(|| None);

#[component]
pub(crate) fn GameScreen() -> Element {
    match GAME_CTRL.read().deref() {
        Some((game, settings)) => {
            rsx! {
                Game { game: game.clone(), settings: settings.clone() },
            }
        },
        None => rsx! {
            GameSetupScreen {}
        },
    }
}