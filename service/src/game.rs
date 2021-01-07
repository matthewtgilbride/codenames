use std::iter::Map;
use std::collections::HashSet;
use std::iter::FromIterator;
use rand::{seq::SliceRandom, thread_rng};
use rand::rngs::ThreadRng;

const BOARD_SIZE: usize = 25;

enum Team {
    Red,
    Blue
}

struct Card {
    covered: bool,
    team: Team,
    word: String
}

struct Player {
    name: String,
}

struct TeamMember {
    is_spy_master: bool,
    player: Player
}

struct Game {
    board: [[Card; 5]; 5],
    players: Map<Team, Vec<Player>>,
    turn: Team,
}
