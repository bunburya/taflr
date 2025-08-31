mod aictrl;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use dioxus::fullstack::once_cell::sync::Lazy;
use dioxus::prelude::*;
use hnefatafl::board::state::{BoardState, MediumBasicBoardState};
use hnefatafl::error::PlayInvalid;
use hnefatafl::game::{Game, GameStatus, MediumBasicGame};
use hnefatafl::play::{ValidPlay, Play};
use serde::{Deserialize, Serialize};
use crate::backend::aictrl::AiController;
use crate::config::GameSettings;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayError {
    NoGame,
    Invalid(PlayInvalid),
}

#[derive(Debug)]
pub(crate) struct GlobalState<T: BoardState> {
    game: Game<T>,
    settings: GameSettings,
    ai_ctrl: AiController<T>
}

static GLOBAL_STATE: Lazy<Arc<Mutex<Option<GlobalState<MediumBasicBoardState>>>>> = Lazy::new(||
    Arc::new(Mutex::new(None))
);

pub(crate) fn init_global_state(
    game: MediumBasicGame,
    settings: GameSettings
) {
    let mut ai_ctrl = AiController::new(
        &game,
        settings.attacker.ai_play_time,
        settings.defender.ai_play_time
    );
    ai_ctrl.request_ai_play(&game);
    *GLOBAL_STATE.lock().unwrap() = Some(GlobalState {
        game,
        settings,
        ai_ctrl
    });
}


#[server]
pub(crate) async fn new_game(
    settings: GameSettings
) -> Result<MediumBasicGame, ServerFnError> {
    let game = MediumBasicGame::new(settings.rules, settings.board.as_str()).unwrap();
    init_global_state(game.clone(), settings);
    Ok(game)
}

#[server]
pub(crate) async fn get_game_clone() -> Result<Option<MediumBasicGame>, ServerFnError> {
    if let Some(g) = GLOBAL_STATE.lock()?.deref() {
        Ok(Some(g.game.clone()))
    } else {
        Ok(None)
    }
}

#[server]
pub(crate) async fn do_play(play: Play) -> Result<Result<MediumBasicGame, PlayError>, ServerFnError> {
    if let Some(gs) = GLOBAL_STATE.lock().unwrap().as_mut() {
        match gs.game.do_play(play) {
            Ok(game_status) => {
                if game_status == GameStatus::Ongoing {
                    gs.ai_ctrl.request_ai_play(&gs.game);
                }
            },
            Err(e) => { return Ok(Err(PlayError::Invalid(e))); }
        }
        Ok(Ok(gs.game.clone()))
    } else {
        Ok(Err(PlayError::NoGame))
    }
}

#[server]
pub(crate) async fn poll_ai_play() -> Result<Option<ValidPlay>, ServerFnError> {
    if let Some(gs) = GLOBAL_STATE.lock().unwrap().as_mut() {
        Ok(gs.ai_ctrl.receive_ai_play(gs.game.state.side_to_play, gs.game.state))
    } else {
        Ok(None)
    }
}

#[server]
pub(crate) async fn get_game_and_settings() -> Result<Option<(MediumBasicGame, GameSettings)>, ServerFnError> {
    match GLOBAL_STATE.lock().unwrap().as_ref() {
        Some(gs) => Ok(Some((gs.game.clone(), gs.settings.clone()))),
        None => Ok(None)
    }
}

#[server]
pub(crate) async fn clear_game() -> Result<(), ServerFnError> {
    *GLOBAL_STATE.lock().unwrap() = None;
    Ok(())
}

#[server]
pub(crate) async fn undo_play() -> Result<Result<MediumBasicGame, PlayError>, ServerFnError> {
    if let Some(gs) = GLOBAL_STATE.lock().unwrap().as_mut() {
        gs.game.undo_last_play();
        gs.ai_ctrl.request_ai_play(&gs.game);
        Ok(Ok(gs.game.clone()))
    } else {
        Ok(Err(PlayError::NoGame))
    }
}