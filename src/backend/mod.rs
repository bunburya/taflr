mod aictrl;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dioxus::fullstack::once_cell::sync::Lazy;
use dioxus::prelude::*;
use hnefatafl::board::state::{BoardState, MediumBasicBoardState};
use hnefatafl::error::PlayInvalid;
use hnefatafl::game::{Game, GameStatus, MediumBasicGame};
use hnefatafl::play::{ValidPlay, Play};
use hnefatafl::rules::Ruleset;
use crate::backend::aictrl::{AiController, AI_CTRL};

struct GlobalState<T: BoardState> {
    game: Game<T>,
    ai_ctrl: AiController<T>
}

static GLOBAL_STATE: Lazy<Arc<Mutex<Option<GlobalState<MediumBasicBoardState>>>>> = Lazy::new(||
    Arc::new(Mutex::new(None))
);

pub(crate) fn init_global_state(
    game: MediumBasicGame,
    attacker_ai_time: Option<Duration>,
    defender_ai_time: Option<Duration>
) {
    let mut ai_ctrl = AiController::new(&game, attacker_ai_time, defender_ai_time);
    ai_ctrl.request_ai_play(&game);
    *GLOBAL_STATE.lock().unwrap() = Some(GlobalState {
        game,
        ai_ctrl
    });
}


#[server]
pub(crate) async fn new_game(
    rules: Ruleset,
    board: String,
    attacker_ai_time: Option<Duration>,
    defender_ai_time: Option<Duration>
) -> Result<MediumBasicGame, ServerFnError> {
    let game = MediumBasicGame::new(rules, board.as_str()).unwrap();
    init_global_state(game.clone(), attacker_ai_time, defender_ai_time);
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
pub(crate) async fn do_play(play: Play) -> Result<Result<MediumBasicGame, PlayInvalid>, ServerFnError> {
    if let Some(gs) = GLOBAL_STATE.lock().unwrap().as_mut() {
        match gs.game.do_play(play) {
            Ok(game_status) => {
                if game_status == GameStatus::Ongoing {
                    gs.ai_ctrl.request_ai_play(&gs.game);
                }
            },
            Err(e) => { return Ok(Err(e)); }
        }
        Ok(Ok(gs.game.clone()))
    } else {
        Ok(Err(PlayInvalid::GameOver))
    }
}

#[server]
pub(crate) async fn poll_ai_play() -> Result<Option<ValidPlay>, ServerFnError> {
    if let Some(gs) = GLOBAL_STATE.lock().unwrap().as_mut() {
        Ok(gs.ai_ctrl.receive_ai_play(gs.game.state.side_to_play))
    } else {
        Ok(None)
    }
}