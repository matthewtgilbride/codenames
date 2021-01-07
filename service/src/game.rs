use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;
use std::iter::Map;

const BOARD_SIZE: usize = 25;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Team {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CardColor {
    Team(Team),
    Neutral,
    Death,
}

const ALL_CARD_COLORS: [CardColor; 4] = [
    CardColor::Team(Team::Blue),
    CardColor::Team(Team::Red),
    CardColor::Neutral,
    CardColor::Death,
];

fn max_card_color(card_color: &CardColor) -> usize {
    match card_color {
        CardColor::Death => 1,
        CardColor::Team(_) => 9,
        _ => 8,
    }
}

#[derive(Debug)]
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

fn generate_board_words(dictionary: HashSet<String>) -> Result<[String; 25], &'static str> {
    if dictionary.len() < (BOARD_SIZE + 1) {
        return Err("dictionary must have at least 26 words");
    }

    let as_vector: Vec<String> = dictionary.into_iter().collect();

    let random_subset: Vec<String> = as_vector
        .choose_multiple(&mut thread_rng(), 25)
        .cloned()
        .collect();

    Ok(random_subset.try_into().unwrap())
}

fn random_color(available_colors: HashSet<CardColor>) -> CardColor {
    let as_vector: Vec<CardColor> = available_colors.into_iter().collect();
    *as_vector.choose(&mut thread_rng()).unwrap()
}

fn card_color_count(partial_board: &Vec<Card>, color: &CardColor) -> usize {
    partial_board
        .iter()
        .filter(|card| card.color == *color)
        .count()
}

fn generate_board(dictionary: HashSet<String>) -> Result<[Card; 25], &'static str> {
    let words = generate_board_words(dictionary).unwrap();
    let mut board: Vec<Card> = Vec::new();

    words.iter().for_each(|word| {
        let available_colors: HashSet<CardColor> = ALL_CARD_COLORS
            .to_vec()
            .iter()
            .filter(|color| card_color_count(&board, color) < max_card_color(color))
            .cloned()
            .collect();

        let color = random_color(available_colors);

        board.push(Card {
            covered: Cell::new(false),
            color,
            word: word.clone(),
        })
    });

    Ok(board.try_into().unwrap())
}
