use std::iter::Map;
use rand::{seq::SliceRandom};

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

fn generate_board_words(dictionary: Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    dictionary.choose_multiple(&mut rng, 25).cloned().collect()
}
