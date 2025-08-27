use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::tiles::Tile;
use crate::components::square::Square;
use crate::gamectrl::GameController;

const BOARD_STYLE: Asset = asset!("assets/css/board.css");

#[component]
pub(crate) fn Board() -> Element {
    let board_state = use_context::<GameController>().game_copy.read().state.board;
    let side_len = board_state.side_len();
    let size_class = match side_len {
        7 => "size7",
        9 => "size9",
        11 => "size11",
        13 => "size13",
        _ => panic!("Unexpected side length")
    };
    let wrapper_size_class = match side_len {
        7 => "wrapper-size7",
        9 => "wrapper-size9",
        11 => "wrapper-size11",
        13 => "wrapper-size13",
        _ => panic!("Unexpected side length"),
    };
    let board_classes = ["board", size_class];
    let wrapper_classes = ["board-wrapper", wrapper_size_class];

    rsx! {
        document::Link { rel: "stylesheet", href: BOARD_STYLE }
        div {
            class: wrapper_classes.join(" "),
            // Left margin
            for row in 0..side_len {
                div {
                    class: "label-cell left-label",
                    style: format!("grid-column: 1 / 2; grid-row: {} / {};", row + 1, row + 2),
                    "{side_len - row}"
                }
            }
            // Bottom margin
            for col in 0..side_len {
                div {
                    class: "label-cell bottom-label",
                    style: format!("grid-row: {} / {}; grid-column: {} / {};", side_len + 1, side_len + 2, col + 2, col + 3),
                    "{(b'A' + col as u8) as char}"
                }
            }
            div {
                class: board_classes.join(" ") + " board-in-wrapper",
                for row in 0..side_len {
                    for col in 0..side_len {
                        Square { tile: Tile::new(side_len - row - 1, col) }
                    }
                }
            }
        }
    }
}
