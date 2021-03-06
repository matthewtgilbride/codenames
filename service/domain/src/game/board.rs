use dyn_clone::DynClone;

use crate::game::card::{Card, CardColor, CardState};
use crate::game::model::Team;
use crate::ServiceResult;

pub const BOARD_SIZE: usize = 25;
pub type Board = [Card; BOARD_SIZE];
pub type BoardState = [CardState; BOARD_SIZE];

#[derive(Clone)]
pub struct BoardService {
    generator: Box<dyn BoardGenerator>,
}

impl BoardService {
    pub fn new(generator: Box<dyn BoardGenerator>) -> BoardService {
        BoardService { generator }
    }

    pub fn new_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)> {
        self.generator.random_board(words)
    }
}

pub trait BoardGenerator: DynClone + Send + Sync {
    fn random_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)>;
}

dyn_clone::clone_trait_object!(BoardGenerator);

pub fn max_card_color(card_color: &CardColor, first_team: &Team) -> usize {
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

pub fn card_color_count(partial_board: &Vec<Card>, color: &CardColor) -> usize {
    partial_board
        .iter()
        .filter(|card| card.color == *color)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::game::card::CardColor;
    use crate::game::model::Team;

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
}
