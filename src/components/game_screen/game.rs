use std::collections::HashSet;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use dioxus::prelude::*;
use hnefatafl::game::MediumBasicGame;
use crate::backend::{do_play, poll_ai_play};
use crate::components::game_screen::board::Board;
use crate::components::game_screen::ctrl_panel::ControlPanel;
use crate::components::game_screen::GAME_CTRL;
use crate::components::game_screen::info_panel::InfoPanel;
use crate::components::game_screen::side_pane::SidePane;
use crate::config::GameSettings;
use crate::gamectrl::GameController;

#[cfg(target_arch = "wasm32")]
async fn async_sleep(ms: u32) {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(ms).await;
}


#[cfg(not(target_arch = "wasm32"))]
async fn async_sleep(ms: u32) {
    use async_std::task::sleep;
    sleep(Duration::from_millis(ms.into())).await;
}

#[component]
pub(crate) fn Game(game: MediumBasicGame, settings: GameSettings) -> Element {
    let game_copy = use_signal(|| game.clone());
    let selected = use_signal(|| None);
    let movable = use_signal(HashSet::new);
    let last_move_time = use_signal(Instant::now);

    let game_ctrl = GameController {
        game_copy,
        selected,
        movable,
        attacker: settings.attacker,
        defender: settings.defender,
        last_move_time,
        game_name: settings.name,
    };

    use_context_provider(|| game_ctrl);

    let _ = use_future(async move || {
        loop {
            let mut game_ctrl = use_context::<GameController>();
            if game_ctrl.ai_play_time().is_some_and(|d| game_ctrl.time_since_last_play() > d) {
                if let Some(vp) = poll_ai_play().await.unwrap() {
                    let g = do_play(vp.play).await.unwrap().unwrap();
                    game_ctrl.game_copy.set(g);
                    game_ctrl.last_move_time.set(Instant::now()); // Now using Signal
                }
            }
            async_sleep(500).await;
        }
    });
    rsx! {
        div {
            class: "game-container",
            Board {}
            SidePane {}
        }

    }
}