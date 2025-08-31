use crate::gamectrl::Player;
use hnefatafl::rules::Ruleset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub(crate) rules: Ruleset,
    pub(crate) board: String,
    pub(crate) variant_name: String,
    pub(crate) attacker: Player,
    pub(crate) defender: Player,
}