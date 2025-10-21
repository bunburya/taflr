use dioxus::prelude::*;
use crate::components::MainMenu;
use crate::components::About;
use crate::components::NewGame;
use crate::components::PlayGame;

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
    #[route("/quit")]
    Quit,
}

#[component]
fn LoadGame() -> Element {
    rsx! {
        h1 { "Load Game" }
    }
}



#[component]
fn Quit() -> Element {
    use_effect(|| {std::process::exit(0);});
    rsx! {
        "Quitting..."
    }
}