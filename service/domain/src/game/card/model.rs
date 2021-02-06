use serde::{Deserialize, Serialize, Serializer};

use crate::game::model::Team;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

impl Serialize for CardColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            CardColor::Team(t) => t.serialize(serializer),
            CardColor::Neutral => serializer.serialize_str("Neutral"),
            CardColor::Death => serializer.serialize_str("Death"),
        }
    }
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
