use std::ops::Deref;
use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::tiles::Tile;
use crate::components::game_screen::piece::PieceIcon;
use crate::gamectrl::GameController;

#[component]
pub(crate) fn Square(tile: Tile) -> Element {
    let game_ui_state = use_context::<GameController>();
    let piece = game_ui_state.game_copy.read().state.board.get_piece(tile);
    let mut classes = vec!["square"];
    let special_tiles = game_ui_state.game_copy.read().logic.board_geo.special_tiles;
    if special_tiles.throne == tile {
        classes.push("throne")
    } else if special_tiles.corners.contains(&tile) {
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