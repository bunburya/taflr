use dioxus::prelude::*;
use hnefatafl::board::state::BoardState;
use hnefatafl::tiles::Tile;
use crate::components::square::Square;
use crate::gamectrl::GameController;

const BOARD_STYLE: Asset = asset!("assets/css/board.css");

#[component]
pub(crate) fn Board() -> Element {
    let board_state = use_context::<GameController>().game_copy.read().state.board;
    let size_class = match board_state.side_len() {
        7 => "size7",
        9 => "size9",
        11 => "size11",
        13 => "size13",
        _ => panic!("Unexpected side length")
    };
    let classes = vec!["board", size_class];
    rsx! {
        document::Link { rel: "stylesheet", href: BOARD_STYLE }
        div {
            class: classes.join(" "),
            for row in 0..board_state.side_len() {
                for col in 0..board_state.side_len() {
                    Square { tile: Tile::new(row, col) }
                }
            }
        }
    }
}
