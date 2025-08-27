//#![cfg(feature = "server")]

use crate::backend::do_play;
use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::game::MediumBasicGame;
use hnefatafl::pieces::Side;
use hnefatafl::play::Play;
use hnefatafl::tiles::Tile;
use std::collections::HashSet;

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct GameController {
    pub(crate) game_copy: Signal<MediumBasicGame>,
    pub(crate) selected: Signal<Option<Tile>>,
    pub(crate) movable: Signal<HashSet<Tile>>,
    pub(crate) attacker_ai_time: Option<Duration>,
    pub(crate) defender_ai_time: Option<Duration>,
    pub(crate) last_move_time: Instant
}

impl GameController {
    pub(crate) fn new(
        game: &MediumBasicGame,
        attacker_ai_time: Option<Duration>,
        defender_ai_time: Option<Duration>,
    ) -> Self {
        Self {
            game_copy: use_signal(|| game.clone()),
            selected: use_signal(|| None),
            movable: use_signal(HashSet::new),
            attacker_ai_time, defender_ai_time,
            last_move_time: Instant::now()
        }
    }

    pub async fn handle_selection(&mut self, tile: Tile) {
        if self.is_ai_turn() {
            return
        }
        if self.selected.read().is_some()
            && self.movable.read().contains(&tile) {
            // unwrap safe because we have just checked
            let from_tile = self.selected.read().unwrap();
            if let Ok(game) = do_play(Play::from_tiles(from_tile, tile).unwrap())
                .await.unwrap() {
                self.game_copy.set(game);
                self.selected.set(None);
                self.movable.set(HashSet::new());
                self.last_move_time = Instant::now();
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

    pub fn is_ai_turn(&self) -> bool {
        match self.game_copy.read().state.side_to_play {
            Side::Attacker => self.attacker_ai_time,
            Side::Defender => self.defender_ai_time
        }.is_some()
    }
    
    pub fn ai_move_time(&self) -> Option<Duration> {
        match self.game_copy.read().state.side_to_play {
            Side::Attacker => self.attacker_ai_time,
            Side::Defender => self.defender_ai_time
        }
    }

    pub fn time_since_last_move(&self) -> Duration {
        Instant::now() - self.last_move_time
    }
}