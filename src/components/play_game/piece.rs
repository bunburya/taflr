use dioxus::prelude::*;
use hnefatafl::pieces::{Piece, PieceType, Side};
use crate::components::play_game::icons::{PIECE_K_WHITE, PIECE_T_BLACK, PIECE_T_WHITE};

fn get_img_src(piece: Piece) -> Asset {
    match (piece.piece_type, piece.side) {
        (PieceType::King, Side::Defender) => PIECE_K_WHITE,
        (PieceType::Soldier, Side::Defender) => PIECE_T_WHITE,
        (PieceType::Soldier, Side::Attacker) => PIECE_T_BLACK,
        _ => panic!("Unexpected piece type")
    }
}

#[component]
pub(crate) fn PieceIcon(piece: Piece) -> Element {
    rsx! {
        img {
            class: "piece",
            src: get_img_src(piece),
        }
    }
}