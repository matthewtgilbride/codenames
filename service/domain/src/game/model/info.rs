use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    game::model::{GameError, Player, Team, Turn, TurnData},
    Lowercase, UniqueError,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    players: HashMap<Lowercase, Player>,
    turns: Vec<Turn>,
}

impl GameInfo {
    pub fn new(name: String, first_team: Team) -> Self {
        Self {
            name,
            players: HashMap::new(),
            turns: vec![Turn::Pending(first_team)],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.values().collect()
    }

    pub fn player(&self, player_name: &str) -> Option<&Player> {
        self.players.get(&Lowercase::new(player_name))
    }

    pub fn turns(&self) -> &[Turn] {
        &self.turns
    }

    pub fn current_turn(&self) -> &Turn {
        self.turns
            .first()
            .expect("Encountered GameInfo with empty list of Turns. This should never happen...")
    }

    pub fn start_turn(
        &self,
        spymaster_name: String,
        clue: (String, usize),
    ) -> Result<Self, GameError> {
        let head = self.current_turn();
        let tail = self.turns[1..].to_vec();
        let maybe_player = self.players.get(&Lowercase::new(spymaster_name.as_str()));

        match (maybe_player, head) {
            (_, Turn::InProgress(_)) => {
                Err(GameError::InvalidTurnState("already started".to_string()))
            }
            (None, _) => Err(GameError::PlayerNotFound(spymaster_name)),
            (Some(player), turn) if player.team != *turn.team() => {
                Err(GameError::PlayerNotFound(spymaster_name))
            }
            (Some(player), _) if player.spymaster_secret.is_none() => {
                Err(GameError::InvalidTurnState("not a spymaster".to_string()))
            }
            (Some(player), _) => Ok(Self {
                turns: [
                    vec![Turn::InProgress(TurnData {
                        spymaster: player.clone(),
                        clue,
                        guesses: Vec::new(),
                    })],
                    tail,
                ]
                .concat(),
                ..self.clone()
            }),
        }
    }

    pub fn end_turn(&self) -> Self {
        let head = self.current_turn().clone();
        let tail = self.turns[1..].to_vec();

        let current_team = match head {
            Turn::Pending(team) => team,
            Turn::InProgress(TurnData {
                spymaster: Player { team, .. },
                ..
            }) => team,
        };

        let new_turn = Turn::Pending(match current_team {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        });

        Self {
            turns: [vec![new_turn, head], tail].concat(),
            ..self.clone()
        }
    }

    pub fn add_player(&self, player: Player) -> Result<Self, GameError> {
        let key = Lowercase::new(player.name.as_str());
        if self.players.contains_key(&key) {
            return Err(GameError::UniquePlayerName(UniqueError {
                entity_name: "player".to_string(),
                field_name: "name".to_string(),
                value: player.name.to_string(),
            }));
        }
        Ok(Self {
            players: [
                self.players.clone().into_iter().collect(),
                vec![(key, player)],
            ]
            .concat()
            .into_iter()
            .collect(),
            ..self.clone()
        })
    }

    pub fn remove_player(&self, player_name: &str) -> Result<Self, GameError> {
        let key = Lowercase::new(player_name);
        if !self.players.contains_key(&key) {
            return Err(GameError::PlayerNotFound(player_name.to_string()));
        }
        Ok(Self {
            players: self
                .players
                .clone()
                .into_iter()
                .filter(|(k, _)| k.clone() != key)
                .collect(),
            ..self.clone()
        })
    }

    pub fn add_guess(&self, guess: (Player, usize)) -> Result<Self, GameError> {
        let head = self.current_turn();
        let tail = self.turns[1..].to_vec();

        match head {
            Turn::Pending(_) => Err(GameError::InvalidTurnState("not started".to_string())),
            Turn::InProgress(TurnData { spymaster, .. })
                if spymaster.team != head.team().clone() =>
            {
                Err(GameError::InvalidGuess(guess.0.name))
            }
            Turn::InProgress(turn_data) => Ok(Self {
                turns: [
                    vec![Turn::InProgress(TurnData {
                        guesses: [vec![guess], turn_data.clone().guesses].concat(),
                        ..turn_data.clone()
                    })],
                    tail,
                ]
                .concat(),
                ..self.clone()
            }),
        }
    }

    pub fn guesses(&self) -> Vec<(Player, usize)> {
        self.turns
            .iter()
            .filter_map(|t| match t {
                Turn::InProgress(data) => Some(data.guesses.clone()),
                _ => None,
            })
            .flatten()
            .collect()
    }
}
