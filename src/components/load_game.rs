use dioxus::prelude::*;
use hnefatafl::pieces::Side;
use crate::components::navbutton::NavButton;
use crate::route::Route;
use crate::sqlite::{DbController, SavedGameInfo};

#[component]
fn SavedGameInfoView(saved_game: SavedGameInfo, to_delete: Signal<Option<i64>>) -> Element {
    let (attacker_class, defender_class) = match saved_game.side_to_play {
        Side::Attacker => ("saved-game-info-player current-player", "saved-game-info-player"),
        Side::Defender => ("saved-game-info-player", "saved-game-info-player current-player"),
    };
    rsx! {
        div {
            class: "saved-game-info",
            div {
                class: "saved-game-name",
                "{saved_game.game_name}"
            }
            div {
                class: "saved-game-variant",
                "{saved_game.variant_name}"
            }
            div {
                class: "saved-game-player-container",
                div {
                    class: attacker_class,
                    "{saved_game.attacker.name} (A)"
                }
                div {
                    class: defender_class,
                    "{saved_game.defender.name} (D)"
                }
            }
            div {
                class: "saved-game-button-container",
                NavButton {
                    class: "saved-game-load-button",
                    route: Route::PlayGame {id: saved_game.id},
                    text: "Load"
                }
                button {
                    class: "action-button saved-game-delete-button",
                    onclick: move |_| {
                        *to_delete.write() = Some(saved_game.id);
                    },
                    "Delete"
                }
            }

        }
    }
}

#[component]
fn SavedGameList(saved_games: Signal<Vec<SavedGameInfo>>, to_delete: Signal<Option<i64>>) -> Element {
    rsx! {
        for saved_game in &*saved_games.read() {
            SavedGameInfoView { saved_game: saved_game.clone(), to_delete: to_delete }
        }
    }
}

#[component]
pub(crate) fn LoadGame() -> Element {

    let to_delete: Signal<Option<i64>> = use_signal(|| None);
    let mut saved_games: Signal<Vec<SavedGameInfo>> = use_signal(Vec::new);

    use_effect(move || {
        let db_ctrl = use_context::<DbController>();
        let id_opt = *to_delete.read();
        if let Some(id) = id_opt {
            spawn(async move {
                db_ctrl.delete_game_from_db(id).await.expect("Failed to delete game from database.");
                saved_games.write().retain(|sg| sg.id != id);
            });
        };
    });

    use_effect(move || {
        let mut db_ctrl = use_context::<DbController>();
        spawn(async move {
            let loaded = db_ctrl.load_saved_game_info().await.expect("Failed to load saved games from database.");
            saved_games.set(loaded);
        });
    });

    // match &*resource.read_unchecked() {
    //     Some(Ok(db_saved_games)) => {
    //         *saved_games.write() = db_saved_games.clone();
    //         rsx! {
    //             document::Stylesheet { href: asset!("/assets/css/load_game.css") }
    //             SavedGameList { saved_games: saved_games, to_delete: to_delete }
    //         }
    //     },
    //     Some(Err(err)) => rsx! { "Error: {err:#?}" },
    //     None => rsx! { "Loading..." },
    // }
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/load_game.css") }
        SavedGameList { saved_games: saved_games, to_delete: to_delete }
    }
}