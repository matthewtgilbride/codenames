use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;

use codenames_domain::game::board::model::Board;
use codenames_domain::game::board::service::BoardGenerator;
use codenames_domain::game::card::model::{Card, CardColor, CardState};
use codenames_domain::game::model::Team;
use codenames_domain::ServiceResult;

#[derive(Clone)]
pub struct BoardGeneratorRand;

impl BoardGeneratorRand {
    fn random_team(&self) -> Team {
        vec![Team::Blue, Team::Red]
            .choose(&mut thread_rng())
            .unwrap()
            .clone()
    }
}

impl BoardGenerator for BoardGeneratorRand {
    fn random_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)> {
        let first_team = self.random_team();

        let mut indices: Vec<usize> = (0..25).collect();
        indices.shuffle(&mut thread_rng());

        let mut initial_board: Vec<CardState> = words
            .iter()
            .map(|word| CardState {
                word: word.clone(),
                color: None,
            })
            .collect();

        indices
            .iter()
            .enumerate()
            .for_each(|(index, &random_index)| {
                let CardState { word, .. } = initial_board[random_index].clone();
                let color = match index {
                    0 => Some(CardColor::Death),
                    i if i < 8 => Some(CardColor::Neutral),
                    i if i < 16 => {
                        if first_team == Team::Blue {
                            Some(CardColor::Team(Team::Red))
                        } else {
                            Some(CardColor::Team(Team::Blue))
                        }
                    }
                    _ => Some(CardColor::Team(first_team)),
                };
                initial_board[random_index] = CardState { word, color }
            });

        let board: Vec<Card> = initial_board
            .iter()
            .map(|CardState { word, color }| Card {
                word: word.clone(),
                color: color.unwrap(),
            })
            .collect();

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
