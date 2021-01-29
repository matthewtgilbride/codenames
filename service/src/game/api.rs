#[cfg(test)]
mod tests {
    use crate::game::board::util::card_color_count;
    use crate::game::tests::rand_dictionary;

    use super::*;

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
