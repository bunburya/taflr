use hnefatafl::rules::Ruleset;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct GameSettings {
    pub(crate) board: String,
    pub(crate) rules: Ruleset,
    pub(crate) attacker_ai_time: Option<Duration>,
    pub(crate) defender_ai_time: Option<Duration>,
}

impl GameSettings {
    pub(crate) fn new(
        board: &str,
        rules: Ruleset,
        attacker_ai_time: Option<Duration>,
        defender_ai_time: Option<Duration>,
    ) -> Self {
        Self {
            board: board.to_owned(),
            rules,
            attacker_ai_time,
            defender_ai_time,
        }
    }

}