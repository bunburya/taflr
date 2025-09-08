#[cfg(target_arch = "wasm32")]
use web_time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use async_std::prelude::StreamExt;
use dioxus::logger::tracing::Instrument;
use dioxus::prelude::*;
use hnefatafl::board::state::MediumBasicBoardState;
use crate::ai::BasicAi;
use crate::aictrl::{compute_ai_play, AiRequest, AI};
use crate::components::game_screen::board::Board;
use crate::components::game_screen::ctrl_panel::ControlPanel;
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
pub(crate) fn Game(settings: GameSettings) -> Element {
    let game_ctrl = GameController::new(settings);

    use_context_provider(move || game_ctrl);

    let ai_coroutine = use_coroutine(|mut rx: UnboundedReceiver<AiRequest<MediumBasicBoardState>>| async move {
        while let Some(request) = rx.next().await {
            let mut game_ctrl = use_context::<GameController>();

            // Compute AI move in background
            let response = compute_ai_play(request).await;
            match response {
                Ok(resp) => {
                    if let Some(ai_move) = game_ctrl.handle_ai_response(resp) {
                        game_ctrl.apply_play(ai_move.play).expect("Invalid AI play");
                    }
                },
                Err(e) => println!("Error: {}", e)
            }
        }
    });

    use_context_provider(|| ai_coroutine);

    use_effect(|| {
        let game_ctrl = use_context::<GameController>();
        if let Some(time_to_play) = game_ctrl.current_player().ai_play_time {
            use_context::<Coroutine<AiRequest<MediumBasicBoardState>>>().send(AiRequest {
                game_state: game_ctrl.game.read().state,
                time_to_play
            });
        }
    });

    rsx! {
        div {
            class: "game-container",
            Board {}
            ControlPanel {}
        }

    }
}