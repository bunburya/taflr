use dioxus::prelude::*;
use crate::components::game_screen::ctrl_panel::ControlPanel;
use crate::components::game_screen::info_panel::InfoPanel;

#[component]
pub(crate) fn SidePane() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/css/game_side_pane.css") }
        div {
            class: "side-pane",
            InfoPanel {}
            ControlPanel {}
        }
    }
}