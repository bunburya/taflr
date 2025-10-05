#[cfg(target_arch = "wasm32")]
use web_time::Instant;
use hnefatafl::game::Game as HnGame;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use async_std::prelude::StreamExt;
use dioxus::prelude::*;
use hnefatafl::aliases::MediumBasicBoardState;
use hnefatafl::game::GameStatus;
use crate::aictrl::{compute_ai_play, AiRequest};
use crate::components;
use crate::components::play_game::board::Board;
use crate::components::play_game::ctrl_panel::ControlPanel;
use crate::config::GameSettings;
use crate::gamectrl::{Action, GameController};
use crate::sqlite::DbController;

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
pub(crate) fn Game(settings: GameSettings, game: HnGame<MediumBasicBoardState>, db_id: i64) -> Element {
    let game_ctrl = GameController::new(settings, game, db_id);

    use_context_provider(move || game_ctrl);

    let ai_coroutine = use_coroutine(|mut rx: UnboundedReceiver<AiRequest<MediumBasicBoardState>>| async move {
        while let Some(request) = rx.next().await {
            let mut game_ctrl = use_context::<GameController<MediumBasicBoardState>>();

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
        let game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
        if let Some(time_to_play) = game_ctrl.current_player().ai_play_time {
            let game_state = game_ctrl.game.read().state;
            if game_state.status == GameStatus::Ongoing {
                use_context::<Coroutine<AiRequest<MediumBasicBoardState>>>().send(AiRequest {
                    game_state,
                    time_to_play
                })
            };
        }
    });

    use_effect(|| {
        let game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
        let action_opt = game_ctrl.last_action.read().as_ref().copied();
        if let Some(action) = action_opt {
            let db_ctrl: DbController = use_context();
            let db_id = game_ctrl.db_id;
            match action {
                Action::Play(play) => {
                    let state = game_ctrl.game.read().state;
                    spawn(async move {
                        db_ctrl.clone().add_turn(db_id, play, state).await
                            .expect("Failed to add turn to database");
                    });
                },
                Action::Undo => {
                    spawn(async move {
                        db_ctrl.clone().undo_turn(db_id).await
                            .expect("Failed to undo turn in database");
                    });
                }
            }
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