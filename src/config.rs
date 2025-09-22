use std::str::FromStr;
use hnefatafl::preset;
use crate::gamectrl::Player;
use hnefatafl::rules::Ruleset;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    pub rules: Ruleset,
    pub starting_board: String,
    pub name: String
}

impl FromStr for Variant {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Copenhagen" => Ok(Self {
                rules: preset::rules::COPENHAGEN,
                starting_board: preset::boards::COPENHAGEN.to_string(),
                name: "Copenhagen".to_string()
            }),
            "Brandubh" => Ok(Self {
                rules: preset::rules::BRANDUBH,
                starting_board: preset::boards::BRANDUBH.to_string(),
                name: "Brandubh".to_string()
            }),
            "Tablut" => Ok(Self {
                rules: preset::rules::TABLUT,
                starting_board: preset::boards::TABLUT.to_string(),
                name: "Tablut".to_string()
            }),
            other => Err(format!("Unknown variant: {}", other))
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameSettings {
    pub(crate) variant: Variant,
    pub(crate) name: String,
    pub(crate) attacker: Player,
    pub(crate) defender: Player,
}