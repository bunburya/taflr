use dioxus::prelude::*;
use hnefatafl::aliases::MediumBasicBoardState;
use hnefatafl::board::state::BoardState;
use hnefatafl::tiles::Tile;
use crate::components::play_game::square::Square;
use crate::gamectrl::GameController;

#[component]
pub(crate) fn Board() -> Element {
    let board_state = use_context::<GameController<MediumBasicBoardState>>().game.read().state.board;
    let side_len = board_state.side_len();

    rsx! {
        div {
            class: "board-frame",
            style: format!("--board-size: {}", side_len),
            for row in 0..side_len {
                div {
                    class: "label-cell left-label",
                    style: format!("grid-column: 1 / 2; grid-row: {} / {};", row + 2, row + 3),
                    "{side_len - row}"
                }
            }
            for col in 0..side_len {
                div {
                    class: "label-cell bottom-label",
                    style: format!("grid-row: {} / {}; grid-column: {} / {};", side_len + 2, side_len + 3, col + 2, col + 3),
                    "{(b'A' + col as u8) as char}"
                }
            }
            div {
                class: "board",
                for row in 0..side_len {
                    for col in 0..side_len {
                        Square { tile: Tile::new(side_len - row - 1, col) }
                    }
                }
            }
        }
    }
}
