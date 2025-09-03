use std::cmp::PartialEq;
use std::ops::Deref;
use std::time::Duration;
use dioxus::prelude::*;
use hnefatafl::preset;
use crate::backend::{get_game_and_settings, new_game};
use crate::components::game_screen::GAME_CTRL;
use crate::config::GameSettings;
use crate::gamectrl::Player;

#[derive(PartialEq)]
enum PlayerType {
    Human,
    AI
}

#[component]
pub(crate) fn GameSetupScreen() -> Element {
    let mut ruleset = use_signal(|| preset::rules::COPENHAGEN);
    let mut board = use_signal(|| preset::boards::COPENHAGEN);
    let mut variant = use_signal(|| String::from("Copenhagen"));

    let mut attacker_name = use_signal(|| "Attacker".to_string());
    let mut attacker_type = use_signal(|| PlayerType::Human);
    let mut attacker_ai_time = use_signal(|| 5u32);

    let mut defender_name = use_signal(|| "Defender".to_string());
    let mut defender_type = use_signal(|| PlayerType::AI);
    let mut defender_ai_time = use_signal(|| 5u32);

    let start_game = move |_: MouseEvent| async move {
        let attacker = Player {
            name: attacker_name.read().deref().clone(),
            ai_play_time: if attacker_type.read().deref() == &PlayerType::AI {
                Some(Duration::from_secs(*attacker_ai_time.read().deref() as u64))
            } else {
                None
            }
        };
        let defender = Player {
            name: defender_name.read().deref().clone(),
            ai_play_time: if defender_type.read().deref() == &PlayerType::AI {
                Some(Duration::from_secs(*defender_ai_time.read().deref() as u64))
            } else {
                None
            }
        };
        let settings = GameSettings {
            rules: *ruleset.read().deref(),
            board: board.read().deref().to_string(),
            name: variant.read().deref().to_string(),
            attacker: attacker.clone(),
            defender: defender.clone()
        };
        // Ask the server to create a new game with the given settings
        new_game(settings).await;
        // Read the created game and settings back from the server
        let (game, settings) = get_game_and_settings().await.unwrap().unwrap();
        // Store the game data and settings client-side
        *GAME_CTRL.write() = Some((game, settings));
    };

    rsx! {
        div {
            class: "game-setup-container",

            h1 {
                class: "setup-title",
                "Game Setup"
            }

            div {
                class: "setup-section",

                h2 {
                    class: "section-title",
                    "Game Rules"
                }

                div {
                    class: "form-group",

                    label {
                        class: "form-label",
                        "Ruleset:"
                    }

                    select {
                        class: "form-select",
                        onchange: move |e| {
                            let sel_str = e.value();
                            let sel_rules = match sel_str.as_str() {
                                "Brandubh" => preset::rules::BRANDUBH,
                                "Tablut" => preset::rules::TABLUT,
                                "Copenhagen" => preset::rules::COPENHAGEN,
                                _ => unreachable!()
                            };
                            ruleset.set(sel_rules);
                            let sel_board = match sel_str.as_str() {
                                "Brandubh" => preset::boards::BRANDUBH,
                                "Tablut" => preset::boards::TABLUT,
                                "Copenhagen" => preset::boards::COPENHAGEN,
                                _ => unreachable!()
                            };
                            board.set(sel_board);
                            variant.set(sel_str);
                        },
                        option { value: "Copenhagen", "Copenhagen" }
                        option { value: "Brandubh", "Brandubh" }
                        option { value: "Tablut", "Tablut" }
                    }
                }
            }

            div {
                class: "players-section",

                // Attacker Player
                div {
                    class: "player-config attacker",

                    h3 {
                        class: "player-title",
                        "Attacker"
                    }

                    div {
                        class: "form-group",

                        label {
                            class: "form-label",
                            "Name:"
                        }

                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{attacker_name}",
                            oninput: move |e| attacker_name.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",

                        label {
                            class: "form-label",
                            "Player Type:"
                        }

                        select {
                            class: "form-select",
                            onchange: move |e| {
                                let value = if e.value() == "AI" { PlayerType::AI } else { PlayerType::Human };
                                attacker_type.set(value);
                            },
                            option { value: "Human", "Human" }
                            option { value: "AI", "AI" }
                        }
                    }

                    if attacker_type.read().deref() == &PlayerType::AI {
                        div {
                            class: "form-group",

                            label {
                                class: "form-label",
                                "AI Think Time (seconds):"
                            }

                            input {
                                class: "form-input",
                                r#type: "number",
                                min: "1",
                                max: "60",
                                value: "{attacker_ai_time}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<u32>() {
                                        attacker_ai_time.set(val);
                                    }
                                }
                            }
                        }
                    }
                }

                // Defender Player
                div {
                    class: "player-config defender",

                    h3 {
                        class: "player-title",
                        "Defender"
                    }

                    div {
                        class: "form-group",

                        label {
                            class: "form-label",
                            "Name:"
                        }

                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{defender_name}",
                            oninput: move |e| defender_name.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",

                        label {
                            class: "form-label",
                            "Player Type:"
                        }

                        select {
                            class: "form-select",
                            onchange: move |e| {
                                let value = if e.value() == "AI" { PlayerType::AI } else { PlayerType::Human };
                                defender_type.set(value);
                            },
                            option { value: "Human", "Human" }
                            option { value: "AI", "AI" }
                        }
                    }

                    if defender_type.read().deref() == &PlayerType::AI {
                        div {
                            class: "form-group",

                            label {
                                class: "form-label",
                                "AI Think Time (seconds):"
                            }

                            input {
                                class: "form-input",
                                r#type: "number",
                                min: "1",
                                max: "60",
                                value: "{defender_ai_time}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<u32>() {
                                        defender_ai_time.set(val);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            button {
                class: "start-game-btn",
                onclick: start_game,
                "Start Game"
            }
        }
    }

}
