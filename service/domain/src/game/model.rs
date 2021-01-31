use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::game::board::model::Board;
use crate::UniqueError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Guess {
    pub board_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub board: Board,
    pub turn: Team,
    pub players: Vec<Player>,
    pub guesses: Vec<Guess>,
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
            .map(|p| Err(GameError::player_name(p.clone()).into()))
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

    pub fn guess(self, guess: Guess) -> GameResult {
        self.guesses
            .iter()
            .find(|Guess { board_index, .. }| *board_index == guess.board_index)
            .map(|g| Err(GameError::guess(g.clone()).into()))
            .unwrap_or_else(|| {
                Ok(Game {
                    guesses: [&[guess], &self.guesses[..]].concat(),
                    ..self.clone()
                })
            })
    }

    pub fn undo_guess(self) -> Game {
        Game {
            guesses: self.guesses[1..].iter().cloned().collect(),
            ..self.clone()
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    PlayerName(UniqueError),
    Guess(UniqueError),
}

impl GameError {
    fn entity_name() -> String {
        "Game".to_string()
    }
    pub fn player_name(player: Player) -> GameError {
        GameError::PlayerName(UniqueError::new(
            GameError::entity_name(),
            "player.name".to_string(),
            player.name,
        ))
    }
    pub fn guess(guess: Guess) -> GameError {
        GameError::Guess(UniqueError::new(
            GameError::entity_name(),
            "guess.board_index".to_string(),
            guess.board_index.to_string(),
        ))
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameError::PlayerName(u) => u.fmt(f),
            GameError::Guess(u) => u.fmt(f),
        }
    }
}

impl Error for GameError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewGameRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuessRequest {
    pub player_name: String,
    pub board_index: usize,
}
