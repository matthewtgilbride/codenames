use std::collections::HashSet;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::board::model::Board;
use crate::game::board::util::{card_color_count, max_card_color};
use crate::game::card::model::{Card, CardColor, ALL_CARD_COLORS};
use crate::game::model::Team;
use crate::model::StdResult;

pub struct Service {
    generator: Box<dyn BoardGenerator>,
}

impl Service {
    pub fn new(generator: Box<dyn BoardGenerator>) -> Service {
        Service { generator }
    }

    pub fn new_board(&self, words: [String; 25]) -> StdResult<(Board, Team)> {
        self.generator.random_board(words)
    }
}

pub trait BoardGenerator {
    fn random_board(&self, words: [String; 25]) -> StdResult<(Board, Team)>;
}

pub struct BoardGeneratorRand;

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
    fn random_board(&self, words: [String; 25]) -> StdResult<(Board, Team)> {
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

#[cfg(test)]
mod tests {
    use crate::dictionary::service::{Service as DictionaryService, WordGeneratorRand};
    use crate::game::board::service::{BoardGeneratorRand, Service};
    use crate::game::board::util::card_color_count;
    use crate::game::card::model::CardColor;
    use crate::game::model::Team;

    #[test]
    fn new_board() {
        let test_dictionary_service =
            DictionaryService::new(Box::new(WordGeneratorRand {})).unwrap();
        let test_service = Service::new(Box::new(BoardGeneratorRand {}));

        let (board, first_team) = test_service
            .new_board(test_dictionary_service.new_word_set().unwrap())
            .unwrap();

        let as_vec = board.to_vec();
        let death_count = card_color_count(&as_vec, &CardColor::Death);
        let neutral_count = card_color_count(&as_vec, &CardColor::Neutral);
        let blue_count = card_color_count(&as_vec, &CardColor::Team(Team::Blue));
        let red_count = card_color_count(&as_vec, &CardColor::Team(Team::Red));
        assert_eq!(1, death_count);
        assert_eq!(7, neutral_count);
        assert_eq!(
            9,
            if first_team == Team::Blue {
                blue_count
            } else {
                red_count
            }
        );
        assert_eq!(
            8,
            if first_team == Team::Blue {
                red_count
            } else {
                blue_count
            }
        );
    }
}
