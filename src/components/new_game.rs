use std::cmp::PartialEq;
use std::ops::Deref;
use std::time::Duration;
use dioxus::prelude::*;
use hnefatafl::preset;
use crate::config::GameSettings;
use crate::error::DbError;
use crate::gamectrl::Player;
use crate::route::Route;
use crate::sqlite::DbController;
use crate::variants::Variant;

#[derive(Debug)]
enum GameCreationStatus {
    Setup,
    Creating(GameSettings),
    Created(i64)
}

const STATUS: GlobalSignal<GameCreationStatus> = Signal::global(|| GameCreationStatus::Setup);

static NAMES: [(&str, &str); 5] = [
    ("Queen Medb", "Cú Chulainn"),
    ("Ragnar Lodbrok", "Charles the Bald"),
    ("Ubba", "Ælla"),
    ("Brian Boru", "Sigtrygg Silkbeard"),
    ("Brennus", "Sulpicius")
];

fn random_player_names() -> (&'static str, &'static str) {
    let time = std::time::SystemTime::now();
    let time = time.duration_since(std::time::UNIX_EPOCH).unwrap();
    let time = time.as_secs() as usize;
    let idx = time % NAMES.len();
    NAMES[idx]
}

fn default_game_name(variant: &str) -> String {
    let dt = chrono::Local::now();
    format!(
        "{} - {}",
        variant,
        dt.format("%Y-%m-%d %H:%M")
    )

}

#[derive(PartialEq)]
enum PlayerType {
    Human,
    AI
}

#[component]
fn CreatingGame(settings: GameSettings) -> Element {
    let db_ctrl = use_context::<DbController>();
    let resource: Resource<Result<i64, DbError>> = use_resource(move || {
        let mut db_ctrl = db_ctrl.clone();
        let settings = settings.clone();
        async move {
            db_ctrl.add_game(settings).await
        }
    });
    match &*resource.read_unchecked() {
        Some(Ok(id)) => {
            println!("Switching status to Created");
            let id = *id;
            use_effect(move || {
                *STATUS.write() = GameCreationStatus::Created(id)
            });
            rsx! { "Game created in database, redirecting..." }
        },
        Some(Err(err)) => rsx! { "Error saving game to database: {err:#?}" },
        None => rsx! { "Saving game to database..."}
    }
}

#[component]
pub(crate) fn GameSetup() -> Element {
    println!("Rendering GameSetupScreen");
    let ruleset = use_signal(|| preset::rules::COPENHAGEN);
    let board = use_signal(|| preset::boards::COPENHAGEN);
    let mut variant = use_signal(|| "Copenhagen".parse::<Variant>().unwrap());
    let default_name = default_game_name(&variant.read().name);
    let mut game_name = use_signal(move || default_name);
    let mut game_name_changed = use_signal(|| false);

    let (att_name, def_name) = random_player_names();

    let mut attacker_name = use_signal(|| att_name.to_string());
    let mut attacker_type = use_signal(|| PlayerType::Human);
    let mut attacker_ai_time = use_signal(|| 5u32);

    let mut defender_name = use_signal(|| def_name.to_string());
    let mut defender_type = use_signal(|| PlayerType::Human);
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
            variant: variant.read().clone(),
            name: game_name.read().deref().to_string(),
            attacker,
            defender
        };
        println!("Switching status to Creating");
        *STATUS.write() = GameCreationStatus::Creating(settings);

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

                div {
                    class: "form-group",
                    label {
                        class: "form-label",
                        "Name:"
                    }
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: "{game_name}",
                        oninput: move |e| {
                            game_name.set(e.value());
                            game_name_changed.set(true);
                        }
                    }
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
                            variant.set(sel_str.parse().unwrap());
                            if !(*game_name_changed.read()) {
                                game_name.set(default_game_name(&variant.read().name));
                            }
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

#[component]
pub(crate) fn NewGame() -> Element {
    println!("Rendering NewGame called, STATUS is {:#?}", STATUS);
    match *STATUS.read() {
        GameCreationStatus::Setup => rsx! {
            GameSetup {}
        },
        GameCreationStatus::Creating(ref settings) => rsx! {
            CreatingGame {settings: settings.clone()}
        },
        GameCreationStatus::Created(id) => {
            println!("Switching status to Setup");
            use_effect(move || {
                *STATUS.write() = GameCreationStatus::Setup;
                let nav = navigator();
                nav.push(Route::PlayGame { id });
            });
            rsx! { "Game created." }
        }
    }
}