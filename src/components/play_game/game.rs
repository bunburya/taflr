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
use crate::game_settings::GameSettings;
use crate::gamectrl::{Action, GameController};
use crate::message::{error_msg, warning_msg};
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
pub(crate) fn GameView(settings: GameSettings, game: HnGame<MediumBasicBoardState>, db_id: i64) -> Element {
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
                        if let Err(e) = game_ctrl.apply_play(ai_move.play) {
                            warning_msg(format!("AI gave invalid play: {e:?}").as_str())
                        }
                    }
                },
                Err(e) => error_msg(format!("Error: {e}").as_str())
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

        let action_opt = game_ctrl.last_action.read().as_ref().copied();
        if let Some(action) = action_opt {
            let db_ctrl: DbController = use_context();
            let db_id = game_ctrl.db_id;
            match action {
                Action::Play(play) => {
                    let state = game_ctrl.game.read().state;
                    spawn(async move {
                        if let Err(e) = db_ctrl.clone().add_turn(db_id, play, state).await {
                            error_msg(format!("Failed to add move to database: {e:?}").as_str());
                            game_ctrl.clone().undo_last_play(true);
                        }
                    });
                },
                Action::Undo => {
                    spawn(async move {
                        // TODO: Once `Game::undo_last_play` in the `hnefatafl` crate is modified to
                        // return the undone play, we should modify this to check the outcome of
                        // `undo_turn` and re-apply the play locally if it failed.
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