use std::{error::Error, fmt, fmt::Formatter};

use crate::{ServiceError, UniqueError};

#[derive(Debug, PartialEq)]
pub enum GameError {
    UniquePlayerName(UniqueError),
    UniqueGuess(UniqueError),
    PlayerNotFound(String),
    WrongTeam(String),
    NotASpymaster(String),
    NotAnOperative(String),
    InvalidGuess(String),
    TurnStarted,
    TurnPending,
}

impl GameError {
    fn entity_name() -> String {
        "Game".to_string()
    }
    pub fn unique_player(player_name: String) -> GameError {
        GameError::UniquePlayerName(UniqueError::new(
            GameError::entity_name(),
            "info.players.name".to_string(),
            player_name,
        ))
    }
    pub fn unique_guess(guess: usize) -> GameError {
        GameError::UniqueGuess(UniqueError::new(
            GameError::entity_name(),
            "info.turns.guesses".to_string(),
            guess.to_string(),
        ))
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameError::UniquePlayerName(u) => u.fmt(f),
            GameError::UniqueGuess(u) => u.fmt(f),
            GameError::PlayerNotFound(name) => write!(f, "{} is not a player in the game", name),
            GameError::WrongTeam(name) =>  write!(f, "{}'s team is not up", name),
            GameError::NotASpymaster(name) =>  write!(f, "{} is not a spy master", name),
            GameError::NotAnOperative(name) =>  write!(f, "{} is not an operative", name),
            GameError::InvalidGuess(msg) => write!(
                f,
                "Guess must be made when a turn is in progress by an operative on the correct team: {}",
                msg
            ),
            GameError::TurnStarted => write!(f, "turn is already started"),
            GameError::TurnPending => write!(f, "turn is not started")
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
