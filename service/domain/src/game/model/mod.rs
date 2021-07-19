use std::{collections::HashMap, convert::TryInto};

pub use board::*;
pub use card::*;
pub use error::*;
use log::info;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use team::*;

mod board;
mod card;
mod error;
mod player;
mod team;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Game {
    Data(GameData),
    State(GameState),
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

pub type GameResult = Result<GameData, GameError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub info: GameInfo,
    pub board: BoardState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameData {
    pub info: GameInfo,
    pub board: Board,
}

impl GameData {
    pub fn new(name: String, board: Board, first_team: Team) -> GameData {
        GameData {
            info: GameInfo {
                name,
                first_team,
                turn: first_team,
                players: HashMap::new(),
                guesses: Vec::new(),
            },
            board,
        }
    }

    pub fn join(self, player: Player) -> GameResult {
        let key = player.name.to_lowercase();
        self.info
            .players
            .get(key.as_str())
            .map(|p| {
                let error = GameError::unique_player(p.clone());
                info!("{}", error);
                Err(error)
            })
            .unwrap_or_else(|| {
                let mut new_players = self.info.players.clone();
                new_players.insert(key, player.clone());
                Ok(GameData {
                    info: self.info.replace_players(new_players),
                    ..self.clone()
                })
            })
    }

    pub fn end_turn(self) -> GameData {
        GameData {
            info: self.info.toggle_turn(),
            ..self.clone()
        }
    }

    pub fn leave(self, player_name: &str) -> GameResult {
        let mut new_players = self.info.players.clone();
        new_players
            .remove(player_name)
            .map(|_| GameData {
                info: self.info.replace_players(new_players.clone()),
                ..self.clone()
            })
            .ok_or_else(|| GameError::PlayerNotFound(player_name.to_string()))
    }

    pub fn guess(self, guess_request: GuessRequest) -> GameResult {
        let maybe_player = self
            .info
            .players
            .get(&guess_request.player_name.to_lowercase());
        match maybe_player {
            None => Err(GameError::PlayerNotFound(guess_request.player_name)),
            Some(Player {
                spymaster_secret: Some(_),
                ..
            }) => Err(GameError::InvalidGuess(format!(
                "{} is a spy master",
                guess_request.player_name
            ))),
            Some(Player { team, .. }) => {
                if *team != self.info.turn {
                    return Err(GameError::InvalidGuess(format!("{} team is not up", team)));
                }
                self.info
                    .guesses
                    .iter()
                    .find(|&index| *index == guess_request.board_index)
                    .map(|g| {
                        let error = GameError::unique_guess(g.clone());
                        info!("{}", error);
                        Err(error)
                    })
                    .unwrap_or_else(|| {
                        Ok(GameData {
                            info: self.info.replace_guesses(
                                [&[guess_request.board_index], &self.info.guesses[..]].concat(),
                            ),
                            ..self.clone()
                        })
                    })
            }
        }
    }

    pub fn undo_guess(self) -> GameData {
        GameData {
            info: self
                .info
                .replace_guesses(self.info.guesses[1..].iter().cloned().collect()),
            ..self.clone()
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
                    .guesses
                    .iter()
                    .find(|board_index| *board_index == &index)
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
    pub name: String,
    pub first_team: Team,
    pub turn: Team,
    pub players: HashMap<String, Player>,
    pub guesses: Vec<usize>,
}

impl GameInfo {
    pub fn replace_players(&self, new_players: HashMap<String, Player>) -> Self {
        Self {
            players: new_players,
            ..self.clone()
        }
    }

    pub fn replace_guesses(&self, new_guesses: Vec<usize>) -> Self {
        Self {
            guesses: new_guesses,
            ..self.clone()
        }
    }

    pub fn toggle_turn(&self) -> Self {
        Self {
            turn: match self.turn {
                Team::Blue => Team::Red,
                _ => Team::Blue,
            },
            ..self.clone()
        }
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
