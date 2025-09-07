use std::ops::Deref;
use dioxus::prelude::*;
use hnefatafl::board::state::{BoardState, MediumBasicBoardState};
use hnefatafl::pieces::BASIC_PIECES;
use hnefatafl::rules::Ruleset;
use hnefatafl::tiles::Tile;
use crate::aictrl::AiRequest;
use crate::components::game_screen::piece::PieceIcon;
use crate::gamectrl::GameController;

fn display_throne(rules: Ruleset) -> bool {
    (!rules.hostile_tiles.throne.is_empty())
        || (!rules.occupiable_tiles.throne.contains_set(&rules.pieces))
        || (!rules.passable_tiles.throne.contains_set(&rules.pieces))
}

#[component]
pub(crate) fn Square(tile: Tile) -> Element {
    let game_ctrl = use_context::<GameController>();
    let piece = game_ctrl.game.read().state.board.get_piece(tile);
    let mut classes = vec!["square"];
    let special_tiles = game_ctrl.game.read().logic.board_geo.special_tiles;
    let rules = game_ctrl.game.read().logic.rules;
    if special_tiles.throne == tile &&  display_throne(rules) {
        // Only highlight throne if there are some special rules applicable to it
        classes.push("throne")
    } else if special_tiles.corners.contains(&tile) && !rules.edge_escape {
        // Only highlight corners in a corner escape game
        classes.push("corner")
    }
    if game_ctrl.selected.read().deref() == &Some(tile) {
        classes.push("selected")
    } else if game_ctrl.movable.read().contains(&tile) {
        classes.push("movable")
    }
    rsx! {
        div {
            class: classes.join(" "),
            onclick: move |_| {
                let mut game_ctrl = use_context::<GameController>();
                game_ctrl.handle_selection(tile);
            },
            { piece.map(|p| Some(rsx! {PieceIcon {piece: p} }))}
        }
    }
}