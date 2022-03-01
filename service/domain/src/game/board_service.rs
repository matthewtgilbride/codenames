use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::{
    game::model::{Board, Card, CardColor, Team},
    ServiceResult,
};

#[derive(Clone)]
pub struct BoardService {
    generator: Box<dyn BoardGenerator>,
}

impl BoardService {
    pub fn new(generator: Box<dyn BoardGenerator>) -> BoardService {
        BoardService { generator }
    }

    pub async fn new_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)> {
        self.generator.random_board(words).await
    }
}

#[async_trait]
pub trait BoardGenerator: DynClone + Send + Sync {
    async fn random_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)>;
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
        .filter(|card| &card.color == color)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::game::model::{CardColor, Team};

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
