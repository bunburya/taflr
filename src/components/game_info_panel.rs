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

#[component]
pub fn PlayHistory(plays: Vec<PlayRecord>) -> Element {
    // Group plays into pairs (attacker play, defender play)
    let play_pairs: Vec<(Option<&PlayRecord>, Option<&PlayRecord>)> = plays
        .chunks(2)
        .map(|chunk| {
            let attacker_play = chunk.get(0);
            let defender_play = chunk.get(1);
            (attacker_play, defender_play)
        })
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: "play-history.css" }
        table {
            class: "play-history",
            thead {
                tr {
                    th { "Play #" }
                    th { "Attacker" }
                    th { "Defender" }
                }
            }
            tbody {
                for (index, (attacker_play, defender_play)) in play_pairs.iter().enumerate() {
                    tr {
                        td { 
                            class: "play-number", 
                            "{index + 1}"
                        }
                        td { 
                            class: "attacker-play",
                            if let Some(pl) = attacker_play {
                                "{pl}"
                            }
                        }
                        td { 
                            class: "defender-play",
                            if let Some(pl) = defender_play {
                                "{pl}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn GameInfoPanel() -> Element {
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
        document::Link { rel: "stylesheet", href: asset!("assets/css/game_info_panel.css") }
        div {
            class: "info-panel",
            div {
                class: "game-name",
                "{game_ctrl.game_name}"
            }
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
