use crate::gamectrl::Player;
use hnefatafl::rules::Ruleset;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameSettings {
    pub(crate) rules: Ruleset,
    pub(crate) board: String,
    pub(crate) name: String,
    pub(crate) attacker: Player,
    pub(crate) defender: Player,
}