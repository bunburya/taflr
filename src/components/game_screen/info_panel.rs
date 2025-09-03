use dioxus::prelude::*;
use hnefatafl::pieces::Side;
use hnefatafl::play::PlayRecord;
use crate::gamectrl::{GameController, Player};

fn display_player(player: &Player, side: Side) -> String {
    format!(
        "{} ({}, {})",
        player.name,
        if side == Side::Attacker { "Attacker" } else { "Defender" },
        if player.is_ai() { "AI" } else { "Human" }
    )
}

/// Display the history of plays (moves) in the current game.
#[component]
fn PlayHistory(plays: Vec<PlayRecord>) -> Element {

    let game_ctrl = use_context::<GameController>();
    let starting_side = game_ctrl.game_copy.read().logic.rules.starting_side;
    let (name1, name2) = if starting_side == Side::Attacker {
        (game_ctrl.attacker.name, game_ctrl.defender.name)
    } else {
        (game_ctrl.defender.name, game_ctrl.attacker.name)
    };
    // Group plays into pairs (attacker play, defender play)
    let play_pairs: Vec<(Option<&PlayRecord>, Option<&PlayRecord>)> = plays
        .chunks(2)
        .map(|chunk| {
            (chunk.first(), chunk.get(1))
        })
        .collect();


    rsx! {
        table {
            class: "play-history",
            thead {
                tr {
                    th { "#" }
                    th { { name1 } }
                    th { { name2 } }
                }
            }
            tbody {
                for (index, (p1, p2)) in play_pairs.iter().enumerate() {
                    tr {
                        td { 
                            class: "play-number", 
                            "{index + 1}"
                        }
                        td { 
                            class: "play-record",
                            if let Some(p) = p1 {
                                "{p}"
                            }
                        }
                        td { 
                            class: "play-record",
                            if let Some(p) = p2 {
                                "{p}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Display information about the current game.
#[component]
pub(crate) fn InfoPanel() -> Element {
    let game_ctrl = use_context::<GameController>();
    let play_history = game_ctrl.game_copy.read().play_history.clone();
    let side_to_play = game_ctrl.game_copy.read().state.side_to_play;
    let mut att_cls = vec!["player-name"];
    let mut def_cls = vec!["player-name"];
    if side_to_play == Side::Attacker {
        att_cls.push("current-player");
    } else {
        def_cls.push("current-player");
    }
    rsx! {
        div {
            class: "info-panel",
            div {
                class: "game-name",
                "{game_ctrl.game_name}"
            }
            hr { }
            div {
                class: att_cls.join(" "),
                "{display_player(&game_ctrl.attacker, Side::Attacker)}"
            }
            div {
                class: def_cls.join(" "),
                "{display_player(&game_ctrl.defender, Side::Defender)}"
            }
            PlayHistory { plays: play_history }
        }
    }
}
