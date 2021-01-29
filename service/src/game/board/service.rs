use crate::game::card::model::{Card, CardColor, ALL_CARD_COLORS};
use crate::game::model::Team;
use std::collections::HashSet;

use crate::game::board::model::Board;
use crate::game::board::util::{card_color_count, max_card_color};
use crate::model::StandardResult;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::convert::TryInto;

pub struct Service {
    generator: Box<dyn BoardGenerator>,
}

impl Service {
    pub fn new(generator: Box<dyn BoardGenerator>) -> Service {
        Service { generator }
    }

    pub fn new_board(&self, words: [String; 25]) -> StandardResult<(Board, Team)> {
        self.generator.random_board(words)
    }
}

pub trait BoardGenerator {
    fn random_board(&self, words: [String; 25]) -> StandardResult<(Board, Team)>;
}

pub struct BoardGeneratorRand {}

impl BoardGeneratorRand {
    fn random_team(&self) -> Team {
        vec![Team::Blue, Team::Red]
            .choose(&mut thread_rng())
            .unwrap()
            .clone()
    }

    fn random_color(&self, available_colors: HashSet<CardColor>) -> CardColor {
        let as_vector: Vec<CardColor> = available_colors.into_iter().collect();
        *as_vector.choose(&mut thread_rng()).unwrap()
    }
}

impl BoardGenerator for BoardGeneratorRand {
    fn random_board(&self, words: [String; 25]) -> StandardResult<(Board, Team)> {
        let first_team = self.random_team();

        let mut board: Vec<Card> = Vec::new();

        words.iter().for_each(|word| {
            let available_colors: HashSet<CardColor> = ALL_CARD_COLORS
                .to_vec()
                .iter()
                .filter(|card_color| {
                    card_color_count(&board, card_color) < max_card_color(card_color, &first_team)
                })
                .cloned()
                .collect();

            let color = self.random_color(available_colors);

            board.push(Card {
                color,
                word: word.clone(),
            })
        });

        Ok((board.try_into().unwrap(), first_team))
    }
}