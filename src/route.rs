use dioxus::prelude::*;
use hnefatafl::aliases::MediumBasicBoardState;
use hnefatafl::game::Game;
use crate::config::GameSettings;
use crate::error::DbError;
use crate::sqlite::DbController;
use crate::components::MainMenu;
use crate::components::About;
use crate::components::NewGame;

#[derive(Routable, Clone, Copy, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    MainMenu,
    #[route("/new_game")]
    NewGame,
    #[route("/load_game")]
    LoadGame,
    #[route("/game/:id")]
    PlayGame { id: i64 },
    #[route("/about")]
    About,
}

#[component]
fn LoadGame() -> Element {
    rsx! {
        h1 { "Load Game" }
    }
}

#[component]
fn PlayGame(id: i64) -> Element {
    let db_ctrl = use_context::<DbController>();
    let resource: Resource<Result<(GameSettings, Game<MediumBasicBoardState>), DbError>> = use_resource(move || {
        let db_ctrl = db_ctrl.clone();
        async move {
            db_ctrl.load_game::<MediumBasicBoardState>(id).await
        }
    });
    match &*resource.read_unchecked() {
        Some(Ok((gs, game))) => {
            rsx! {
                "Gi"
            }
        },
        Some(Err(err)) => rsx! { "Error: {err:#?}" },
        None => rsx! { "Loading..." },
    }
}