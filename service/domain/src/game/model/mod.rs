use std::{collections::HashMap, convert::TryInto};

pub use board::*;
pub use card::*;
pub use error::*;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use team::*;
pub use turn::*;

use crate::{Lowercase, UniqueError};

mod board;
mod card;
mod error;
mod player;
mod team;
mod turn;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Game {
    State(GameState),
    Data(GameData),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    #[serde(flatten)]
    pub info: GameInfo,
    pub board: BoardState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameData {
    #[serde(flatten)]
    pub info: GameInfo,
    pub board: Board,
}

pub type GameResult = Result<GameData, GameError>;

impl GameData {
    pub fn new(name: String, board: Board, first_team: Team) -> GameData {
        GameData {
            info: GameInfo {
                name,
                players: HashMap::new(),
                turns: vec![Turn::Pending(first_team)],
            },
            board,
        }
    }

    pub fn join(self, player: Player) -> GameResult {
        let info = self.info.add_player(player)?;
        Ok(Self {
            info,
            ..self.clone()
        })
    }

    pub fn end_turn(self) -> GameData {
        GameData {
            info: self.info.end_turn(),
            ..self.clone()
        }
    }

    pub fn leave(self, player_name: &str) -> GameResult {
        let info = self.info.remove_player(player_name)?;
        Ok(Self {
            info,
            ..self.clone()
        })
    }

    pub fn guess(self, guess: (&str, usize)) -> GameResult {
        let (player_name, board_index) = guess;
        let player = self
            .info
            .guesses()
            .iter()
            .find(|&(_, index)| *index == board_index)
            .map(|_| {
                Err(GameError::UniqueGuess(UniqueError::new(
                    "Game".to_string(),
                    "guesses".to_string(),
                    board_index.to_string(),
                )))
            })
            .unwrap_or(
                self.info
                    .players
                    .get(&Lowercase::new(player_name))
                    .ok_or(GameError::PlayerNotFound(player_name.to_string())),
            )?;

        let info = self.info.add_guess((player.clone(), board_index))?;

        Ok(Self {
            info,
            ..self.clone()
        })
    }
}

impl From<(Player, GameData)> for Game {
    fn from(
        (
            Player {
                spymaster_secret, ..
            },
            g,
        ): (Player, GameData),
    ) -> Self {
        match spymaster_secret {
            Some(_) => Game::Data(g.clone()),
            _ => Game::State(g.clone().into()),
        }
    }
}

impl Into<GameState> for GameData {
    fn into(self) -> GameState {
        let cards: Vec<CardState> = self
            .board
            .iter()
            .enumerate()
            .map(|(index, card)| {
                let maybe_card_color = self
                    .info
                    .guesses()
                    .iter()
                    .find(|(_, board_index)| *board_index == index)
                    .map(|_| card.color);
                CardState {
                    color: maybe_card_color,
                    word: card.clone().word,
                }
            })
            .collect();
        let board = cards.try_into().unwrap();
        GameState {
            info: self.info,
            board,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    players: HashMap<Lowercase, Player>,
    turns: Vec<Turn>,
}

impl GameInfo {
    pub fn new(name: &str, first_team: Team) -> Self {
        Self {
            name: name.to_lowercase(),
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
        match self.players.contains_key(&key) {
            true => Err(GameError::UniquePlayerName(UniqueError {
                entity_name: "player".to_string(),
                field_name: "name".to_string(),
                value: player.name.to_string(),
            })),
            false => Ok(Self {
                players: [
                    self.players.clone().into_iter().collect(),
                    vec![(key, player)],
                ]
                .concat()
                .into_iter()
                .collect(),
                ..self.clone()
            }),
        }
    }

    pub fn remove_player(&self, player_name: &str) -> Result<Self, GameError> {
        let key = Lowercase::new(player_name);
        match self.players.contains_key(&key) {
            false => Err(GameError::PlayerNotFound(player_name.to_string())),
            true => Ok(Self {
                players: self
                    .players
                    .clone()
                    .into_iter()
                    .filter(|(k, _)| k.clone() != key)
                    .collect(),
                ..self.clone()
            }),
        }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewGameRequest {
    pub game_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRequest {
    pub player_name: String,
}

impl PlayerRequest {
    pub fn new(player_name: &str) -> Self {
        Self {
            player_name: player_name.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuessRequest {
    pub player_name: String,
    pub board_index: usize,
}

impl GuessRequest {
    pub fn new(player_name: &str, board_index: usize) -> Self {
        Self {
            player_name: player_name.to_string(),
            board_index,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameList {
    pub games: Vec<String>,
}
