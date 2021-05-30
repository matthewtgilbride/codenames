use std::fmt;
use std::fmt::Display;

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::game::model::Team;

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
            &cc => serializer.serialize_str(cc.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for CardColor {
    fn deserialize<D>(deserializer: D) -> Result<CardColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CardColorVisitor)
    }
}

struct CardColorVisitor;

impl<'de> Visitor<'de> for CardColorVisitor {
    type Value = CardColor;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#"the literal string "Blue", "Red", "Neutral", or "Death""#)
    }

    fn visit_str<E>(self, value: &str) -> Result<CardColor, E>
    where
        E: de::Error,
    {
        match value {
            "Blue" => Ok(CardColor::Team(Team::Blue)),
            "Red" => Ok(CardColor::Team(Team::Red)),
            "Neutral" => Ok(CardColor::Neutral),
            "Death" => Ok(CardColor::Death),
            s => Err(E::custom(format!("Unknown string value: {}", s))),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CardState {
    pub color: Option<CardColor>,
    pub word: String,
}

#[cfg(test)]
mod tests {
    use crate::game::card::CardColor;
    use crate::game::model::Team;

    #[test]
    fn serialize_card_color() {
        let blue = CardColor::Team(Team::Blue);
        let j = serde_json::to_string(&blue).unwrap();
        assert_eq!(j, r#""Blue""#);
        let neutral = CardColor::Neutral;
        let k = serde_json::to_string(&neutral).unwrap();
        assert_eq!(k, r#""Neutral""#)
    }

    #[test]
    fn deserialize_card_color() {
        let blue = r#""Blue""#;
        let result: CardColor = serde_json::from_str(blue).unwrap();
        assert_eq!(result, CardColor::Team(Team::Blue))
    }
}
