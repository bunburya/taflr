use std::ops::Deref;
use dioxus::prelude::*;
use crate::backend::{clear_game, get_game_and_settings, undo_play};
use crate::components::game_screen::GAME_CTRL;
use crate::gamectrl::GameController;

#[component]
pub(crate) fn ControlPanel() -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                clear_game().await;
                *GAME_CTRL.write() = None;
            },
            "Quit Game"
        }
        button {
            onclick: move |_| async move {
                use_context::<GameController>().undo_last_play().await;
            },
            "Undo"
        }
    }
}