use serde::{Deserialize, Serialize};

use crate::game::model::{Player, Team};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnData {
    pub spymaster: Player,
    pub clue: (String, usize),
    pub guesses: Vec<(Player, usize)>,
}

impl TurnData {
    pub fn new(spymaster: Player, clue: (String, usize)) -> Self {
        Self {
            spymaster,
            clue,
            guesses: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Turn {
    Pending(Team),
    Started(TurnData),
}

impl Turn {
    pub fn team(&self) -> &Team {
        match self {
            Turn::Pending(team) => team,
            Turn::Started(TurnData {
                spymaster: Player { team, .. },
                ..
            }) => team,
        }
    }
}
