use std::collections::HashMap;

pub const BOARD_SIZE: usize = 25;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub covered: bool,
    pub color: CardColor,
    pub word: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub team: Team,
    pub name: String,
    pub is_spy_master: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Guess {
    pub team: Team,
    pub board_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    pub board: [Card; 25],
    pub turn: Team,
    pub players: Vec<Player>,
    pub guesses: Vec<Guess>,
}

impl Game {
    pub fn new(board: [Card; 25], turn: Team, players: Vec<Player>) -> Game {
        Game {
            board,
            turn,
            players,
            guesses: Vec::new(),
        }
    }

    pub fn add_player(self, player: Player) -> Game {
        Game {
            players: [&[player], &self.players[..]].concat(),
            ..self.clone()
        }
    }

    pub fn remove_player(self, player: Player) -> Game {
        Game {
            players: self
                .players
                .iter()
                .filter(|&p| p != &player)
                .cloned()
                .collect(),
            ..self.clone()
        }
    }

    pub fn add_guess(self, guess: Guess) -> Game {
        Game {
            guesses: [&[guess], &self.guesses[..]].concat(),
            ..self.clone()
        }
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
    use super::*;
    use crate::game::tests::rand_game;

    #[test]
    fn add_player() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game.add_player(Player {
            team: Team::Blue,
            name: "quz".to_string(),
            is_spy_master: false,
        });

        assert_eq!(game_clone.players.len() + 1, updated_game.players.len())
    }

    #[test]
    fn remove_player() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game.remove_player(Player {
            team: Team::Blue,
            name: "foo".to_string(),
            is_spy_master: true,
        });

        assert_eq!(game_clone.players.len() - 1, updated_game.players.len())
    }
}
