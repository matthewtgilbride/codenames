use crate::game::card::model::{Card, CardColor};
use crate::game::model::Team;

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
