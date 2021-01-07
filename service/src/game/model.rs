use std::cell::Cell;
use std::iter::Map;

pub const BOARD_SIZE: usize = 25;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

#[derive(Debug)]
pub struct Card {
    pub covered: Cell<bool>,
    pub color: CardColor,
    pub word: String,
}

pub struct Player {
    name: String,
}

pub struct TeamMember {
    is_spy_master: bool,
    player: Player,
}

pub struct Game {
    board: [Card; 25],
    players: Map<Team, Vec<Player>>,
    turn: Cell<Team>,
}
