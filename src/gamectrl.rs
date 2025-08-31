//#![cfg(feature = "server")]

use crate::backend::{do_play, undo_play};
use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::game::MediumBasicGame;
use hnefatafl::pieces::Side;
use hnefatafl::play::Play;
use hnefatafl::tiles::Tile;
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

/// Information about a player
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
    /// A copy of the ongoing game (*not* the "source of truth"), wrapped in a signal.
    pub(crate) game_copy: Signal<MediumBasicGame>,
    /// The selected tile, if any, wrapped in a signal.
    pub(crate) selected: Signal<Option<Tile>>,
    /// The set of tiles that are accessible from the selected tile, wrapped in a signal.
    pub(crate) movable: Signal<HashSet<Tile>>,
    /// Information about the attacking player.
    pub(crate) attacker: Player,
    /// Information about the defending player.
    pub(crate) defender: Player,
    /// The time the last move was made by either player, wrapped in a signal.
    pub(crate) last_move_time: Signal<Instant>,
    /// The name of the game.
    pub(crate) game_name: String,
}

impl GameController {
    /// Handle the selection of a tile by the user, including, where necessary, processing a player
    /// move.
    pub async fn handle_selection(&mut self, tile: Tile) {
        if self.is_ai_turn() {
            return
        }
        if self.selected.read().is_some()
            && self.movable.read().contains(&tile) {
            // unwrap safe because we have just checked
            let from_tile = self.selected.read().unwrap();
            if let Ok(game) = do_play(Play::from_tiles(from_tile, tile).unwrap()).await.unwrap() {
                self.game_copy.set(game);
                self.selected.set(None);
                self.movable.set(HashSet::new());
                self.last_move_time.set(Instant::now());
            }
        } else {
            let game = self.game_copy.read();
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
        match self.game_copy.read().state.side_to_play {
            Side::Attacker => &self.attacker,
            Side::Defender => &self.defender
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

    pub async fn undo_last_play(&mut self) {
        if let Ok(Ok(g)) = undo_play().await {
            self.game_copy.set(g);
            self.selected.set(None);
            self.movable.set(HashSet::new());
            self.last_move_time.set(Instant::now());
        }
    }
}