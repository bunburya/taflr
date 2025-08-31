use crate::gamectrl::Player;
use hnefatafl::rules::Ruleset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct GameSettings {
    pub(crate) rules: Ruleset,
    pub(crate) board: String,
    pub(crate) name: String,
    pub(crate) attacker: Player,
    pub(crate) defender: Player,
}