use std::iter::Map;
use rand::{seq::SliceRandom};
use std::collections::HashSet;
use std::iter::FromIterator;

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

fn generate_board_words(dictionary: HashSet<String>) -> HashSet<String> {
    let mut rng = rand::thread_rng();
    let as_vector: Vec<String> = dictionary.into_iter().collect();
    let random_subset: Vec<String> = as_vector.choose_multiple(&mut rng, 25).cloned().collect();
    HashSet::from_iter(random_subset.iter().cloned())
}
