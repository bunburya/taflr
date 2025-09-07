mod board;
mod piece;
mod icons;
mod square;
mod game;
mod info_panel;
pub mod setup;
mod ctrl_panel;
mod side_pane;

use std::ops::Deref;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use crate::components::game_screen::game::Game;
use crate::components::GameSetupScreen;
use crate::config::GameSettings;

//static GAME_CTRL: GlobalSignal<Option<(MediumBasicGame, GameSettings)>> = Signal::global(|| None);
static GAME_SETTINGS: GlobalSignal<Option<GameSettings>> = Signal::global(|| None);

#[component]
pub(crate) fn GameScreen() -> Element {

    match GAME_SETTINGS.read().deref() {
        Some(settings) => {
            rsx! {
                Game { settings: settings.clone() },
            }
        },
        None => rsx! {
            GameSetupScreen {}
        },
    }
}