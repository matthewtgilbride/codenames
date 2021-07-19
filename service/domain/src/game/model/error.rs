use std::{error::Error, fmt, fmt::Formatter};

use crate::{game::model::Player, ServiceError, UniqueError};

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
