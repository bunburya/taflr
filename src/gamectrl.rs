use crate::ai::{Ai, BasicAi};
use crate::aictrl::{AiResponse, AI};
use crate::config::GameSettings;
use dioxus::prelude::*;
use hnefatafl::aliases::{MediumBasicBoardState, MediumBasicGame};
use hnefatafl::board::state::BoardState;
use hnefatafl::error::PlayInvalid;
use hnefatafl::game::state::GameState;
use hnefatafl::game::GameStatus;
use hnefatafl::pieces::Side;
use hnefatafl::play::{Play, ValidPlay};
use hnefatafl::tiles::Tile;
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

/// Information about a player
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Player {
    /// Player's name
    pub(crate) name: String,
    /// If this player is an AI, the amount of time it has to make a play. If the player is not an
    /// AI, this should be `None`.
    pub(crate) ai_play_time: Option<Duration>
}

impl Player {
    /// Whether this player is an AI.
    pub(crate) fn is_ai(&self) -> bool {
        self.ai_play_time.is_some()
    }
}

/// This struct contains certain information required to display the game and has methods to
/// interact with the game.
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct GameController {
    /// The game settings.
    pub(crate) settings: GameSettings,
    /// A copy of the ongoing game (*not* the "source of truth"), wrapped in a signal.
    pub(crate) game: Signal<MediumBasicGame>,
    /// The selected tile, if any, wrapped in a signal.
    pub(crate) selected: Signal<Option<Tile>>,
    /// The set of tiles that are accessible from the selected tile, wrapped in a signal.
    pub(crate) movable: Signal<HashSet<Tile>>,
    /// The time the last move was made by either player, wrapped in a signal.
    pub(crate) last_move_time: Signal<Instant>,
    /// The `id` of the game in the database.
    pub(crate) db_id: i64,
}

impl GameController {

    pub(crate) fn new(settings: GameSettings, game: MediumBasicGame, db_id: i64) -> Self {
        use_effect(move || {
            *AI.write() = Some(BasicAi::new(game.logic));
        });

        Self {
            settings,
            game: use_signal(move || game),
            selected: use_signal(|| None),
            movable: use_signal(HashSet::new),
            last_move_time: use_signal(Instant::now),
            db_id
        }
    }

    pub(crate) fn apply_play(&mut self, play: Play) -> Result<GameStatus, PlayInvalid> {
        let play_res = self.game.write().do_play(play);
        if play_res.is_ok() {
            self.selected.set(None);
            self.movable.set(HashSet::new());
            self.last_move_time.set(Instant::now());
        }
        play_res
    }

    /// Handle the selection of a tile by the user, including, where necessary, processing a player
    /// move.
    pub fn handle_selection(&mut self, tile: Tile) {
        if self.is_ai_turn() {
            return
        }
        if self.selected.read().is_some()
            && self.movable.read().contains(&tile) {
            // unwrap safe because we have just checked
            let from_tile = self.selected.read().unwrap();
            let play_res = self.game.write()
                .do_play(Play::from_tiles(from_tile, tile).unwrap());
            if let Ok(_) = play_res {
                self.selected.set(None);
                self.movable.set(HashSet::new());
                self.last_move_time.set(Instant::now());
            }
        } else {
            let game = self.game.read();
            let piece = game.state.board.get_piece(tile);
            if let Some(piece) = piece {
                if piece.side == game.state.side_to_play {
                    self.selected.set(Some(tile));
                    if let Ok(iter) = game.iter_plays(tile) {
                        self.movable.set(iter.map(|p| p.play.to()).collect())
                    }
                }
            }
        }
    }

    /// The player whose turn it is.
    pub fn current_player(&self) -> &Player {
        match self.game.read().state.side_to_play {
            Side::Attacker => &self.settings.attacker,
            Side::Defender => &self.settings.defender
        }
    }

    /// Whether the current player is an AI.
    pub fn is_ai_turn(&self) -> bool {
        self.current_player().is_ai()
    }

    /// The amount of time the current player has to make a play, if the current player is an AI, or
    /// `None` otherwise.
    pub fn ai_play_time(&self) -> Option<Duration> {
        self.current_player().ai_play_time
    }

    /// The amount of time since the last move was made.
    pub fn time_since_last_play(&self) -> Duration {
        Instant::now() - *self.last_move_time.read()
    }

    pub async fn request_ai_play(&mut self) -> Option<(ValidPlay, GameState<MediumBasicBoardState>)> {
        let game_state = self.game.read().state;
        let ai_time = self.current_player().ai_play_time;
        if let Some(ttp) = ai_time {
            tokio::task::spawn_blocking(move || {
                if let Ok((vp, _)) = AI.write().as_mut().unwrap().next_play(&game_state, ttp) {
                    Some((vp, game_state))
                } else {
                    None
                }
            }).await.unwrap()
        } else {
            None
        }
    }

    pub fn handle_ai_response(&mut self, ai_resp: AiResponse<MediumBasicBoardState>) -> Option<ValidPlay> {
        if ai_resp.game_state == self.game.read().state {
            Some(ai_resp.play)
        } else {
            None
        }
    }

    pub fn undo_last_play(&mut self) {
        self.game.write().undo_last_play();
        self.selected.set(None);
        self.movable.set(HashSet::new());
        self.last_move_time.set(Instant::now());
    }

    pub(crate) fn add_to_db(&self) {

    }

}