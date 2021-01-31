use serde::{Deserialize, Serialize};

use crate::game::model::Team;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

pub const ALL_CARD_COLORS: [CardColor; 4] = [
    CardColor::Team(Team::Blue),
    CardColor::Team(Team::Red),
    CardColor::Neutral,
    CardColor::Death,
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub color: CardColor,
    pub word: String,
}
