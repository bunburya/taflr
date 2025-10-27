use dioxus::prelude::*;
use hnefatafl::aliases::MediumBasicBoardState;
use hnefatafl::game::Game;
use crate::components::play_game::game::GameView;
use crate::game_settings::GameSettings;
use crate::error::DbError;
use crate::sqlite::DbController;

mod board;
mod piece;
mod icons;
mod square;
pub(crate) mod game;
mod ctrl_panel;

#[component]
pub(crate) fn PlayGame(id: i64) -> Element {
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
                GameView { settings: gs.clone(), game: game.clone(), db_id: id }
            }
        },
        Some(Err(err)) => rsx! { "Error: {err:#?}" },
        None => rsx! { "Loading..." },
    }
}