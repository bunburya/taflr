mod aictrl;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use dioxus::fullstack::once_cell::sync::Lazy;
use dioxus::prelude::*;
use hnefatafl::board::state::{BoardState, MediumBasicBoardState};
use hnefatafl::error::PlayInvalid;
use hnefatafl::game::{Game, GameStatus, MediumBasicGame};
use hnefatafl::play::{ValidPlay, Play};
use hnefatafl::preset;
use hnefatafl::rules::Ruleset;
use crate::backend::aictrl::{AiController, AI_CTRL};

static GAME: Lazy<Arc<Mutex<Option<MediumBasicGame>>>> = Lazy::new(||
    Arc::new(Mutex::new(None))
);

pub(crate) fn init_global_state(
    game: &MediumBasicGame,
    attacker_ai_time: Option<Duration>,
    defender_ai_time: Option<Duration>
) {
    let mut ai_ctrl = AiController::new(&game, attacker_ai_time, defender_ai_time);
    ai_ctrl.request_ai_play(game);
    *AI_CTRL.lock().unwrap() = ai_ctrl;
    *GAME.lock().unwrap() = Some(game.clone());
}


#[server]
pub(crate) async fn new_game(
    board: String,
    rules: Ruleset,
    attacker_ai_time: Option<Duration>,
    defender_ai: Option<Duration>
) -> Result<MediumBasicGame, ServerFnError> {
    let game = MediumBasicGame::new(board, rules).unwrap();
    init_global_state(&game, attacker_ai, defender_ai, time_to_play);
    Ok(game)
}

#[server]
pub(crate) async fn get_game() -> Result<Option<MediumBasicGame>, ServerFnError> {
    Ok(GAME.lock()?.clone())
}

#[server]
pub(crate) async fn do_play(play: Play) -> Result<Result<MediumBasicGame, PlayInvalid>, ServerFnError> {
    if let Some(g) = GAME.lock().unwrap().as_mut() {
        match g.do_play(play) {
            Ok(game_status) => {
                if game_status == GameStatus::Ongoing {
                    AI_CTRL.lock().unwrap().request_ai_play(g);
                }
            },
            Err(e) => { return Ok(Err(e)); }
        }
        Ok(Ok(g.clone()))
    } else {
        Ok(Err(PlayInvalid::GameOver))
    }
}

#[server]
pub(crate) async fn poll_ai_play() -> Result<Option<ValidPlay>, ServerFnError> {
    if let Some(g) = GAME.lock().unwrap().deref() {
        Ok(AI_CTRL.lock().unwrap().receive_ai_play(g.state.side_to_play))
    } else {
        Ok(None)
    }
}