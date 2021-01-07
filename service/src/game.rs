use rand::seq::SliceRandom;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::iter::Map;

const BOARD_SIZE: usize = 25;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Team {
    Red,
    Blue,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

fn max_card_colors(card_color: CardColor) -> usize {
    match card_color {
        CardColor::Death => 1,
        CardColor::Team(_) => 9,
        _ => 8,
    }
}

struct Card {
    covered: Cell<bool>,
    color: CardColor,
    word: String,
}

struct Player {
    name: String,
}

struct TeamMember {
    is_spy_master: bool,
    player: Player,
}

struct Game {
    board: [Card; 25],
    players: Map<Team, Vec<Player>>,
    turn: Team,
}

fn generate_board_words(dictionary: HashSet<String>) -> Result<HashSet<String>, &'static str> {
    if dictionary.len() < (BOARD_SIZE + 1) {
        return Err("dictionary must have at least 26 words");
    }
    let mut rng = rand::thread_rng();
    let as_vector: Vec<String> = dictionary.into_iter().collect();
    let random_subset: Vec<String> = as_vector.choose_multiple(&mut rng, 25).cloned().collect();
    Ok(HashSet::from_iter(random_subset.iter().cloned()))
}

fn random_color(available_colors: HashSet<CardColor>) -> CardColor {
    let mut rng = rand::thread_rng();
    let as_vector: Vec<CardColor> = available_colors.into_iter().collect();
    *as_vector.choose(&mut rng).unwrap_or(&CardColor::Neutral)
}

fn card_color_count(partial_board: Vec<Card>, color: CardColor) -> usize {
    partial_board
        .iter()
        .filter(|card| card.color == color)
        .count()
}
