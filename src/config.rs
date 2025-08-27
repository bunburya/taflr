use std::time::Duration;
use hnefatafl::rules::Ruleset;
use crate::backend::new_game;
use crate::gamectrl::GameController;

pub(crate) struct GameSettings {
    board: String,
    rules: Ruleset,
    attacker_ai_time: Option<Duration>,
    defender_ai_time: Option<Duration>,
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

    pub(crate) async fn init(&self) {
        let game = new_game(
            self.board.clone(),
            self.rules,
            self.attacker_ai_time,
            self.defender_ai_time
        ).await.unwrap();
        GameController::new()
    }

}