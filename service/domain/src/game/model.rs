use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use log::info;
use serde::{Deserialize, Serialize};

use crate::game::board::model::{Board, BoardState};
use crate::game::card::model::CardState;
use crate::game::model::GameError::InvalidGuess;
use crate::UniqueError;

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub team: Team,
    pub name: String,
    pub is_spy_master: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub board: Board,
    pub turn: Team,
    pub players: Vec<Player>,
    pub guesses: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameState {
    pub name: String,
    pub board: BoardState,
    pub turn: Team,
    pub players: Vec<Player>,
    pub guesses: Vec<usize>,
}

pub type GameResult = Result<Game, GameError>;

impl Game {
    pub fn new(name: String, board: Board, turn: Team) -> Game {
        Game {
            name,
            board,
            turn,
            players: Vec::new(),
            guesses: Vec::new(),
        }
    }

    pub fn join(self, player: Player) -> GameResult {
        self.players
            .iter()
            .find(|Player { name, .. }| *name == player.name)
            .map(|p| {
                let error = GameError::unique_player(p.clone());
                info!("{}", error);
                Err(error)
            })
            .unwrap_or_else(|| {
                Ok(Game {
                    players: [&[player], &self.players[..]].concat(),
                    ..self.clone()
                })
            })
    }

    pub fn end_turn(self) -> Game {
        Game {
            turn: match self.turn {
                Team::Blue => Team::Red,
                _ => Team::Blue,
            },
            ..self.clone()
        }
    }

    pub fn leave(self, player_name: &str) -> Game {
        Game {
            players: self
                .players
                .iter()
                .filter(|Player { name, .. }| *name != player_name)
                .cloned()
                .collect(),
            ..self.clone()
        }
    }

    pub fn guess(self, guess_request: GuessRequest) -> GameResult {
        self.players
            .iter()
            .cloned()
            .find(
                |Player {
                     name,
                     is_spy_master,
                     team,
                     ..
                 }| {
                    *name == guess_request.player_name
                        && *is_spy_master == false
                        && *team == self.turn
                },
            )
            .map_or_else(
                || {
                    info!("{}", InvalidGuess);
                    Err(InvalidGuess)
                },
                |_| {
                    self.guesses
                        .iter()
                        .find(|&index| *index == guess_request.board_index)
                        .map(|g| {
                            let error = GameError::unique_guess(g.clone());
                            info!("{}", error);
                            Err(error)
                        })
                        .unwrap_or_else(|| {
                            Ok(Game {
                                guesses: [&[guess_request.board_index], &self.guesses[..]].concat(),
                                ..self.clone()
                            })
                        })
                },
            )
    }

    pub fn undo_guess(self) -> Game {
        Game {
            guesses: self.guesses[1..].iter().cloned().collect(),
            ..self.clone()
        }
    }
}

impl Into<BoardState> for Game {
    fn into(self) -> BoardState {
        let cards: Vec<CardState> = self
            .board
            .iter()
            .enumerate()
            .map(|(index, card)| {
                let maybe_card_color = self
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

impl Into<GameState> for Game {
    fn into(self) -> GameState {
        GameState {
            name: self.clone().name,
            board: self.clone().into(),
            turn: self.clone().turn,
            players: self.players,
            guesses: self.guesses,
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    UniquePlayerName(UniqueError),
    PlayerNotFound(String),
    UniqueGuess(UniqueError),
    InvalidGuess,
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
            GameError::InvalidGuess => write!(f, "guess must be made by a valid player in the game (by name), on the team that matches the game's current turn, who is not a spy master")
        }
    }
}

impl Error for GameError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewGameRequest {
    pub game_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveRequest {
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuessRequest {
    pub player_name: String,
    pub board_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameList {
    pub games: Vec<String>,
}
