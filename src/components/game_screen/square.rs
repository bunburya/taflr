use std::ops::Deref;
use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::rules::ThroneRule;
use hnefatafl::tiles::Tile;
use crate::components::game_screen::piece::PieceIcon;
use crate::gamectrl::GameController;

#[component]
pub(crate) fn Square(tile: Tile) -> Element {
    let game_ui_state = use_context::<GameController>();
    let piece = game_ui_state.game_copy.read().state.board.get_piece(tile);
    let mut classes = vec!["square"];
    let special_tiles = game_ui_state.game_copy.read().logic.board_geo.special_tiles;
    let rules = game_ui_state.game_copy.read().logic.rules;
    if special_tiles.throne == tile &&
        (!rules.hostility.throne.is_empty() || rules.throne_movement != ThroneRule::NoRule) {
        // Only highlight throne if there are some special rules applicable to it
        classes.push("throne")
    } else if special_tiles.corners.contains(&tile) && !rules.edge_escape {
        // Only highlight corners in a corner escape game
        classes.push("corner")
    }
    if game_ui_state.selected.read().deref() == &Some(tile) {
        classes.push("selected")
    } else if game_ui_state.movable.read().contains(&tile) {
        classes.push("movable")
    }
    rsx! {
        div {
            class: classes.join(" "),
            onclick: move |_| async move { use_context::<GameController>().handle_selection(tile).await } ,
            { piece.map(|p| Some(rsx! {PieceIcon {piece: p} }))}
        }
    }
}