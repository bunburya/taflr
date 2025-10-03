mod about;
mod mainmenu;
mod style;
mod play_game;
mod new_game;

use dioxus::prelude::{GlobalSignal, Signal};
use crate::config::GameSettings;

// Main menu components
pub(crate) use new_game::NewGame;
pub(crate) use about::About;
pub(crate) use mainmenu::MainMenu;

pub(crate) static GAME_SETTINGS: GlobalSignal<Option<GameSettings>> = Signal::global(|| None);
