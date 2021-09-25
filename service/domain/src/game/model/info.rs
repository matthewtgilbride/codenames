use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    game::model::{GameError, Player, Team, Turn, TurnData},
    Lowercase,
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

    pub fn guesses(&self) -> Vec<(Player, usize)> {
        self.turns
            .iter()
            .filter_map(|t| match t {
                Turn::Started(data) => Some(data.guesses.clone()),
                _ => None,
            })
            .flatten()
            .collect()
    }

    pub fn start_turn(
        self,
        spymaster_name: String,
        clue: (String, usize),
    ) -> Result<Self, GameError> {
        let maybe_player = self.players.get(&Lowercase::new(spymaster_name.as_str()));
        let current_turn = self.current_turn();
        let previous_turns = self.turns[1..].to_vec();

        match (maybe_player, current_turn) {
            (_, Turn::Started(_)) => Err(GameError::TurnStarted),

            (None, _) => Err(GameError::PlayerNotFound(spymaster_name)),

            (Some(player), turn) if &player.team != turn.team() => {
                Err(GameError::WrongTeam(spymaster_name))
            }

            (Some(player), _) if player.spymaster_secret.is_none() => {
                Err(GameError::NotASpymaster(spymaster_name))
            }

            (Some(player), _) => Ok(Self {
                turns: [
                    vec![Turn::Started(TurnData::new(player.clone(), clue))],
                    previous_turns,
                ]
                .concat(),
                ..self.clone()
            }),
        }
    }

    pub fn end_turn(self) -> Self {
        let head = self.current_turn().clone();
        let tail = self.turns[1..].to_vec();

        let current_team = match head {
            Turn::Pending(team) => team,
            Turn::Started(TurnData {
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

    pub fn add_player(self, player: Player) -> Result<Self, GameError> {
        let key = Lowercase::new(player.name.as_str());
        if self.players.contains_key(&key) {
            return Err(GameError::unique_player(player.name));
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

    pub fn remove_player(self, player_name: &str) -> Result<Self, GameError> {
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

    pub fn add_guess(self, guess: (&str, usize)) -> Result<Self, GameError> {
        let (player_name, board_index) = guess;
        let player = self.players.get(&Lowercase::new(player_name));
        let head = self.current_turn();
        let tail = self.turns[1..].to_vec();

        match (player, head) {
            (_, Turn::Pending(_)) => Err(GameError::TurnPending),
            (None, _) => Err(GameError::PlayerNotFound(player_name.to_string())),
            (Some(player), Turn::Started(TurnData { spymaster, .. }))
                if spymaster.team != player.team =>
            {
                Err(GameError::WrongTeam(spymaster.clone().name))
            }
            (
                Some(Player {
                    spymaster_secret: Some(_),
                    name,
                    ..
                }),
                _,
            ) => Err(GameError::NotAnOperative(name.clone())),
            (
                Some(player),
                Turn::Started(TurnData {
                    clue,
                    guesses,
                    spymaster,
                }),
            ) => Ok(Self {
                turns: [
                    vec![Turn::Started(TurnData {
                        guesses: [vec![(player.clone(), board_index)], guesses.clone()].concat(),
                        clue: clue.clone(),
                        spymaster: spymaster.clone(),
                    })],
                    tail,
                ]
                .concat(),
                ..self.clone()
            }),
        }
    }
}

#[cfg(test)]
#[path = "info_tests.rs"]
mod tests;
