pub mod model {
    use std::cell::Cell;
    use std::iter::Map;

    pub const BOARD_SIZE: usize = 25;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Team {
        Red,
        Blue,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CardColor {
        Team(Team),
        Neutral,
        Death,
    }

    #[derive(Debug)]
    pub struct Card {
        pub covered: Cell<bool>,
        pub color: CardColor,
        pub word: String,
    }

    pub struct Player {
        name: String,
    }

    pub struct TeamMember {
        is_spy_master: bool,
        player: Player,
    }

    pub struct Game {
        board: [Card; 25],
        players: Map<Team, Vec<Player>>,
        turn: Team,
    }
}

pub mod api {
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

    pub fn generate_board(words: [String; 25]) -> Result<[Card; 25], &'static str> {
        let mut board: Vec<Card> = Vec::new();

        words.iter().for_each(|word| {
            let available_colors: HashSet<CardColor> = ALL_CARD_COLORS
                .to_vec()
                .iter()
                .filter(|card_color| {
                    card_color_count(&board, card_color) < max_card_color(card_color)
                })
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

    fn max_card_color(card_color: &CardColor) -> usize {
        match card_color {
            CardColor::Death => 1,
            CardColor::Team(_) => 9,
            _ => 8,
        }
    }
}
