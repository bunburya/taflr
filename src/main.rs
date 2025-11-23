mod components;
mod ai;
mod gamectrl;
mod game_settings;
mod aictrl;
mod sqlite;
mod error;
mod variants;
mod route;
mod message;

use dioxus::prelude::*;
use crate::components::StatusBar;
use crate::error::DbError;
use crate::route::Route;
use crate::sqlite::DbController;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn Style() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/variables.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Stylesheet { href: asset!("/assets/css/game.css") }
        document::Stylesheet { href: asset!("/assets/css/game_setup.css") }
        document::Stylesheet { href: asset!("/assets/css/header_bar.css") }
        document::Stylesheet { href: asset!("/assets/css/about.css") }
    }
}

// #[component]
// fn App() -> Element {
//     let resource: Resource<Result<DbController, DbError>> = use_resource(async || { DbController::new().await });
//     match &*resource.read_unchecked() {
//         Some(Ok(db_ctrl)) => {
//             use_context_provider(move || db_ctrl.clone());
//             rsx! {
//                 div {
//                     Style {}
//                     GameScreen {}
//                 }
//             }
//         },
//         Some(Err(err)) => rsx! { "Error: {err:#?}" },
//         None => rsx! { "Loading..." },
//     }
// }

#[component]
fn App() -> Element {
    let resource: Resource<Result<DbController, DbError>> = use_resource(async || { DbController::new().await });
    match &*resource.read_unchecked() {
        Some(Ok(db_ctrl)) => {
            use_context_provider(move || db_ctrl.clone());
            rsx! {
                Style {}
                Router::<Route> {}
                StatusBar {}
            }
        },
        Some(Err(err)) => rsx! { "Error: {err:#?}" },
        None => rsx! { "Connecting to database..." },
    }

}

