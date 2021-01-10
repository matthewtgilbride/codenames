use super::model::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::Cell;
use std::collections::HashSet;
use std::convert::TryInto;

pub fn generate_board_words(dictionary: HashSet<String>) -> Result<[String; 25], &'static str> {
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

pub fn generate_board(words: [String; 25]) -> Result<([Card; 25], Team), &'static str> {
    let first_team: Team = vec![Team::Blue, Team::Red]
        .choose(&mut thread_rng())
        .unwrap()
        .clone();

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

        let color = random_color(available_colors);

        board.push(Card {
            covered: false,
            color,
            word: word.clone(),
        })
    });

    Ok((board.try_into().unwrap(), first_team))
}

fn random_color(available_colors: HashSet<CardColor>) -> CardColor {
    let as_vector: Vec<CardColor> = available_colors.into_iter().collect();
    *as_vector.choose(&mut thread_rng()).unwrap()
}

const ALL_CARD_COLORS: [CardColor; 4] = [
    CardColor::Team(Team::Blue),
    CardColor::Team(Team::Red),
    CardColor::Neutral,
    CardColor::Death,
];

fn card_color_count(partial_board: &Vec<Card>, color: &CardColor) -> usize {
    partial_board
        .iter()
        .filter(|card| card.color == *color)
        .count()
}

fn max_card_color(card_color: &CardColor, first_team: &Team) -> usize {
    match card_color {
        CardColor::Team(team) => {
            if team == first_team {
                return 9;
            }
            8
        }
        CardColor::Neutral => 7,
        CardColor::Death => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::tests::rand_dictionary;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use std::iter;
    use std::iter::Repeat;

    #[test]
    fn max_card_color() {
        assert_eq!(
            9,
            super::max_card_color(&CardColor::Team(Team::Blue), &Team::Blue)
        );
        assert_eq!(
            8,
            super::max_card_color(&CardColor::Team(Team::Red), &Team::Blue)
        );
        assert_eq!(7, super::max_card_color(&CardColor::Neutral, &Team::Blue));
        assert_eq!(1, super::max_card_color(&CardColor::Death, &Team::Blue));
    }

    #[test]
    fn generate_board() {
        let result = super::generate_board(generate_board_words(rand_dictionary(50)).unwrap());
        let (board, first_team) = result.unwrap();
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
