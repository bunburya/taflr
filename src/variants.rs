use hnefatafl::rules::Ruleset;

pub(crate) const OOTB_VARIANTS: [(Ruleset, &str, &str); 4] = [
    (hnefatafl::preset::rules::COPENHAGEN, hnefatafl::preset::boards::COPENHAGEN, "Copenhagen"),
    (hnefatafl::preset::rules::TABLUT, hnefatafl::preset::boards::TABLUT, "Tablut"),
    (hnefatafl::preset::rules::BRANDUBH, hnefatafl::preset::boards::BRANDUBH, "Brandubh"),
    (hnefatafl::preset::rules::MAGPIE, hnefatafl::preset::boards::MAGPIE, "Magpie"),
];