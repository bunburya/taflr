use std::str::FromStr;
use hnefatafl::preset;
use hnefatafl::rules::Ruleset;

pub(crate) const OOTB_VARIANTS: [(Ruleset, &str, &str); 4] = [
    (preset::rules::COPENHAGEN, preset::boards::COPENHAGEN, "Copenhagen"),
    (preset::rules::TABLUT, preset::boards::TABLUT, "Tablut"),
    (preset::rules::BRANDUBH, preset::boards::BRANDUBH, "Brandubh"),
    (preset::rules::MAGPIE, preset::boards::MAGPIE, "Magpie"),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    pub rules: Ruleset,
    pub starting_board: String,
    pub name: String,
    pub is_custom: bool,
}

impl FromStr for Variant {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Copenhagen" => Ok(Self {
                rules: preset::rules::COPENHAGEN,
                starting_board: preset::boards::COPENHAGEN.to_string(),
                name: "Copenhagen".to_string(),
                is_custom: false
            }),
            "Brandubh" => Ok(Self {
                rules: preset::rules::BRANDUBH,
                starting_board: preset::boards::BRANDUBH.to_string(),
                name: "Brandubh".to_string(),
                is_custom: false
            }),
            "Tablut" => Ok(Self {
                rules: preset::rules::TABLUT,
                starting_board: preset::boards::TABLUT.to_string(),
                name: "Tablut".to_string(),
                is_custom: false
            }),
            "Magpie" => Ok(Self {
                rules: preset::rules::MAGPIE,
                starting_board: preset::boards::MAGPIE.to_string(),
                name: "Magpie".to_string(),
                is_custom: false
            }),
            other => Err(format!("Unknown variant: {}", other))
        }
    }
}