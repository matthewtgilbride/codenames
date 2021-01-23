use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json;

pub const BOARD_SIZE: usize = 25;

pub enum DictionaryType {
    Default,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub color: CardColor,
    pub word: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub team: Team,
    pub name: String,
    pub is_spy_master: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Guess {
    pub team: Team,
    pub board_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub board: [Card; 25],
    pub turn: Team,
    pub players: Vec<Player>,
    pub guesses: Vec<Guess>,
}

impl Game {
    pub fn new(name: String, board: [Card; 25], turn: Team) -> Result<Game, String> {
        Ok(Game {
            name,
            board,
            turn,
            players: Vec::new(),
            guesses: Vec::new(),
        })
    }

    pub fn join(self, player: Player) -> Result<Game, String> {
        self.players
            .iter()
            .find(|Player { name, .. }| *name == player.name)
            .map(|_| Err("player names must be unique".to_string()))
            .unwrap_or_else(|| {
                Ok(Game {
                    players: [&[player], &self.players[..]].concat(),
                    ..self.clone()
                })
            })
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

    pub fn guess(self, guess: Guess) -> Result<Game, String> {
        self.guesses
            .iter()
            .find(|Guess { board_index, .. }| *board_index == guess.board_index)
            .map(|_| Err("card has already been guessed".to_string()))
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

#[cfg(test)]
mod tests {
    use crate::game::tests::rand_game;

    use super::*;

    #[test]
    fn join() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game
            .join(Player {
                team: Team::Blue,
                name: "quz".to_string(),
                is_spy_master: false,
            })
            .unwrap();

        assert_eq!(game_clone.players.len() + 1, updated_game.players.len());

        let failed_update = updated_game.join(Player {
            team: Team::Red,
            name: "quz".to_string(),
            is_spy_master: true,
        });

        assert!(failed_update.is_err())
    }

    #[test]
    fn leave() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game.leave("foo");

        assert_eq!(game_clone.players.len() - 1, updated_game.players.len())
    }

    #[test]
    fn serialize() {
        let game = rand_game();
        let j = serde_json::to_string(&game).unwrap();

        println!("{}", j);
    }

    #[test]
    fn guess() {
        let game: Game = rand_game();
        let game_clone = game.clone();
        let updated_game = game
            .guess(Guess {
                team: Team::Blue,
                board_index: 0,
            })
            .unwrap();

        assert_eq!(game_clone.guesses.len() + 1, updated_game.guesses.len());

        let failed_update = updated_game.guess(Guess {
            team: Team::Blue,
            board_index: 0,
        });

        assert!(failed_update.is_err())
    }
}
