use std::convert::TryInto;

pub use board::*;
pub use card::*;
pub use error::*;
pub use info::*;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use team::*;
pub use turn::*;

use crate::UniqueError;

mod board;
mod card;
mod error;
mod info;
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
            info: GameInfo::new(name, first_team),
            board,
        }
    }

    pub fn join(self, player: Player) -> GameResult {
        let GameData { board, info } = self;
        let info = info.add_player(player)?;
        Ok(Self { info, board })
    }

    pub fn start_turn(self, spymaster_name: String, clue: (String, usize)) -> GameResult {
        let GameData { board, info } = self;
        let info = info.start_turn(spymaster_name, clue)?;
        Ok(Self { info, board })
    }

    pub fn end_turn(self) -> GameData {
        let GameData { board, info } = self;
        Self {
            info: info.end_turn(),
            board,
        }
    }

    pub fn leave(self, player_name: &str) -> GameResult {
        let GameData { board, info } = self;
        let info = info.remove_player(player_name)?;
        Ok(Self { info, board })
    }

    pub fn guess(self, guess: (&str, usize)) -> GameResult {
        let (player_name, board_index) = guess;
        let GameData { info, board } = self;
        let player = info
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
                info.player(player_name)
                    .ok_or(GameError::PlayerNotFound(player_name.to_string())),
            )?;

        let info = info.clone().add_guess((player.clone(), board_index))?;

        Ok(Self { info, board })
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
