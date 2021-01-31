use std::collections::HashSet;
use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use codenames_domain::game::board::model::Board;
use codenames_domain::game::board::service::BoardGenerator;
use codenames_domain::game::board::util::{card_color_count, max_card_color};
use codenames_domain::game::card::model::{Card, CardColor, ALL_CARD_COLORS};
use codenames_domain::game::model::Team;
use codenames_domain::StdResult;

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
    use codenames_domain::dictionary::service::Service as DictionaryService;
    use codenames_domain::game::board::service::Service;
    use codenames_domain::game::board::util::card_color_count;
    use codenames_domain::game::card::model::CardColor;
    use codenames_domain::game::model::Team;

    use crate::dictionary::service::WordGeneratorRand;
    use crate::game::board::service::BoardGeneratorRand;

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
