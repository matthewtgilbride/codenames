use serde::{Deserialize, Serialize};

use crate::game::model::{Player, Team};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnData {
    pub spymaster: Player,
    pub clue: (String, usize),
    pub guesses: Vec<(Player, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Turn {
    Pending(Team),
    InProgress(TurnData),
}

impl Turn {
    pub fn team(&self) -> &Team {
        match self {
            Turn::Pending(team) => team,
            Turn::InProgress(TurnData {
                spymaster: Player { team, .. },
                ..
            }) => team,
        }
    }
}
