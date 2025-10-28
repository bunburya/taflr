mod about;
mod mainmenu;
mod style;
mod play_game;
mod new_game;
mod navbutton;
mod load_game;
mod header_bar;
mod status_bar;

use dioxus::prelude::{GlobalSignal, Signal};
use crate::game_settings::GameSettings;

// Main menu components
pub(crate) use new_game::NewGame;
pub(crate) use about::About;
pub(crate) use mainmenu::MainMenu;
pub(crate) use play_game::PlayGame;
pub(crate) use load_game::LoadGame;

pub(crate) use status_bar::StatusBar;

pub(crate) static GAME_SETTINGS: GlobalSignal<Option<GameSettings>> = Signal::global(|| None);
