use crate::gamectrl::Player;
use crate::variants::Variant;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameSettings {
    pub(crate) variant: Variant,
    pub(crate) name: String,
    pub(crate) attacker: Player,
    pub(crate) defender: Player,
}