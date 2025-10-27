use dioxus::prelude::*;
use hnefatafl::aliases::{MediumBasicBoardState, MediumPlayRecord};
use hnefatafl::pieces::Side;
use crate::components::navbutton::NavButton;
use crate::gamectrl::GameController;
use crate::route::Route;

#[component]
fn PlayerTh(side: Side) -> Element {
    let game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
    let player = if side == Side::Attacker {
        game_ctrl.settings.attacker
    } else {
        game_ctrl.settings.defender
    };
    let status_str = format!(
        "({}, {})",
        if side == Side::Attacker { "Attacker" } else { "Defender" },
        if player.is_ai() { "AI" } else { "Human" }
    );
    let cls = if game_ctrl.game.read().state.side_to_play == side {
        "current-player"
    } else {
        ""
    };
    rsx! {
        th {
            class: cls,
            "{player.name}"
            br {}
            "{status_str}"
        }
    }

}

/// Display the history of plays (moves) in the current game.
#[component]
fn PlayHistory(plays: Vec<MediumPlayRecord>) -> Element {

    let game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
    let starting_side = game_ctrl.game.read().logic.rules.starting_side;

    // Group plays into pairs (attacker play, defender play)
    let play_pairs: Vec<(Option<&MediumPlayRecord>, Option<&MediumPlayRecord>)> = plays
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
                    if starting_side == Side::Attacker {
                        PlayerTh { side: Side::Attacker }
                        PlayerTh { side: Side::Defender }
                    } else {
                        PlayerTh { side: Side::Defender }
                        PlayerTh { side: Side::Attacker }
                    }
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

#[component]
pub(crate) fn ControlPanel() -> Element {

    let game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
    let play_history = game_ctrl.game.read().play_history.clone();
    let side_to_play = game_ctrl.game.read().state.side_to_play;
    let mut att_cls = vec!["player-name"];
    let mut def_cls = vec!["player-name"];
    if side_to_play == Side::Attacker {
        att_cls.push("current-player");
    } else {
        def_cls.push("current-player");
    }
    rsx! {
        div {
            class: "ctrl-panel",
            div {
                class: "game-name",
                "{game_ctrl.settings.name}"
            }
            hr {}
            div {
                class: "ctrl-btn-container",
                NavButton {
                    class: "ctrl-btn",
                    replace: true,
                    route: Route::MainMenu,
                    text: "Quit Game"
                }
                button {
                    class: "action-button ctrl-btn nav-button",
                    onclick: |_| {
                        let mut game_ctrl = use_context::<GameController<MediumBasicBoardState>>();
                        game_ctrl.undo_last_play();
                    },
                    "Undo"
                }
            }
            PlayHistory { plays: play_history }
        }
    }
}