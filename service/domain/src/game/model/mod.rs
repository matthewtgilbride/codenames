pub mod player;

use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use log::info;
use serde::{Deserialize, Serialize};

use crate::game::board::{Board, BoardState};
use crate::game::card::CardState;
pub use crate::game::model::player::Player;
use crate::game::model::GameError::{InvalidGuess, PlayerNotFound};
use crate::{ServiceError, UniqueError};

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameData {
    pub info: GameInfo,
    pub board: Board,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub info: GameInfo,
    pub board: BoardState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameVariant {
    Data(GameData),
    State(GameState),
}

impl From<(Player, GameData)> for GameVariant {
    fn from(
        (
            Player {
                spymaster_secret, ..
            },
            g,
        ): (Player, GameData),
    ) -> Self {
        match spymaster_secret {
            Some(_) => GameVariant::Data(g.clone()),
            _ => GameVariant::State(g.clone().into()),
        }
    }
}

pub type GameResult = Result<GameData, GameError>;

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
        self.info.players
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
            .ok_or_else(|| PlayerNotFound(player_name.to_string()))
    }

    pub fn guess(self, guess_request: GuessRequest) -> GameResult {
        let maybe_player = self.info.players.get(&guess_request.player_name.to_lowercase());
        match maybe_player {
            None => Err(PlayerNotFound(guess_request.player_name)),
            Some(Player {
                spymaster_secret: Some(_),
                ..
            }) => Err(InvalidGuess(format!(
                "{} is a spy master",
                guess_request.player_name
            ))),
            Some(Player { team, .. }) => {
                if *team != self.info.turn {
                    return Err(InvalidGuess(format!("{} team is not up", team)));
                }
                self.info.guesses
                    .iter()
                    .find(|&index| *index == guess_request.board_index)
                    .map(|g| {
                        let error = GameError::unique_guess(g.clone());
                        info!("{}", error);
                        Err(error)
                    })
                    .unwrap_or_else(|| {
                        Ok(GameData {
                            info: self.info.replace_guesses([&[guess_request.board_index], &self.info.guesses[..]].concat()),
                            ..self.clone()
                        })
                    })
            }
        }
    }

    pub fn undo_guess(self) -> GameData {
        GameData {
            info: self.info.replace_guesses(self.info.guesses[1..].iter().cloned().collect()),
            ..self.clone()
        }
    }
}

impl Into<BoardState> for GameData {
    fn into(self) -> BoardState {
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
        cards.try_into().unwrap()
    }
}

impl Into<GameState> for GameData {
    fn into(self) -> GameState {
        let GameData {
            info,
            ..
        } = self.clone();
        GameState {
            info,
            board: self.clone().into(),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    UniquePlayerName(UniqueError),
    PlayerNotFound(String),
    UniqueGuess(UniqueError),
    InvalidGuess(String),
}

impl GameError {
    fn entity_name() -> String {
        "Game".to_string()
    }
    pub fn unique_player(player: Player) -> GameError {
        GameError::UniquePlayerName(UniqueError::new(
            GameError::entity_name(),
            "player.name".to_string(),
            player.name,
        ))
    }
    pub fn unique_guess(guess: usize) -> GameError {
        GameError::UniqueGuess(UniqueError::new(
            GameError::entity_name(),
            "guesses".to_string(),
            guess.to_string(),
        ))
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameError::UniquePlayerName(u) => u.fmt(f),
            GameError::UniqueGuess(u) => u.fmt(f),
            GameError::PlayerNotFound(name) => write!(f, "player not found: {}", name),
            GameError::InvalidGuess(msg) => write!(
                f,
                "Guess must be made by an operative on the current team: {}",
                msg
            ),
        }
    }
}

impl From<GameError> for ServiceError {
    fn from(game_error: GameError) -> Self {
        match game_error {
            GameError::UniquePlayerName(ue) => ServiceError::BadRequest(ue.to_string()),
            GameError::UniqueGuess(ue) => ServiceError::BadRequest(ue.to_string()),
            e => ServiceError::BadRequest(e.to_string()),
        }
    }
}

impl Error for GameError {}

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
