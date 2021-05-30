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
use crate::game::model::GameError::{InvalidGuess, PlayerNotFound};
use crate::{ServiceError, UniqueError};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub board: Board,
    pub first_team: Team,
    pub turn: Team,
    pub players: HashMap<String, Player>,
    pub guesses: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub name: String,
    pub board: BoardState,
    pub turn: Team,
    pub players: HashMap<String, Player>,
    pub guesses: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameVariant {
    Data(Game),
    State(GameState),
}

impl From<(Player, Game)> for GameVariant {
    fn from((Player { is_spy_master, .. }, g): (Player, Game)) -> Self {
        match is_spy_master {
            true => GameVariant::Data(g.clone()),
            false => GameVariant::State(g.clone().into()),
        }
    }
}

pub type GameResult = Result<Game, GameError>;

impl Game {
    pub fn new(name: String, board: Board, first_team: Team) -> Game {
        Game {
            name,
            board,
            first_team,
            turn: first_team,
            players: HashMap::new(),
            guesses: Vec::new(),
        }
    }

    pub fn join(self, player: Player) -> GameResult {
        let key = player.name.to_lowercase();
        self.players
            .get(key.as_str())
            .map(|p| {
                let error = GameError::unique_player(p.clone());
                info!("{}", error);
                Err(error)
            })
            .unwrap_or_else(|| {
                let mut new_players = self.players.clone();
                new_players.insert(key, player.clone());
                Ok(Game {
                    players: new_players,
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

    pub fn leave(self, player_name: &str) -> GameResult {
        let mut new_players = self.players.clone();
        new_players
            .remove(player_name)
            .map(|_| Game {
                players: new_players.clone(),
                ..self.clone()
            })
            .ok_or_else(|| PlayerNotFound(player_name.to_string()))
    }

    pub fn guess(self, guess_request: GuessRequest) -> GameResult {
        let maybe_player = self.players.get(&guess_request.player_name.to_lowercase());
        match maybe_player {
            None => Err(PlayerNotFound(guess_request.player_name)),
            Some(Player {
                team,
                is_spy_master,
                ..
            }) => {
                if *is_spy_master {
                    return Err(InvalidGuess(format!(
                        "{} is a spy master",
                        guess_request.player_name
                    )));
                }
                if *team != self.turn {
                    return Err(InvalidGuess(format!("{} team is not up", team)));
                }
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
            }
        }
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
        let Game {
            name,
            turn,
            players,
            guesses,
            ..
        } = self.clone();
        GameState {
            name,
            turn,
            players,
            guesses,
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
