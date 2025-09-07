use dioxus::prelude::*;
use hnefatafl::board::state::MediumBasicBoardState;
use crate::aictrl::AiRequest;
use crate::components::game_screen::GAME_SETTINGS;
use crate::gamectrl::GameController;

#[component]
pub(crate) fn ControlPanel() -> Element {
    rsx! {
        button {
            onclick: |_| {
                *GAME_SETTINGS.write() = None
            },
            "Quit Game"
        }
        button {
            onclick: |_| {
                let mut game_ctrl = use_context::<GameController>();
                game_ctrl.undo_last_play();
            },
            "Undo"
        }
    }
}