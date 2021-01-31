#[cfg(test)]
mod tests {
    use domain::game::model::{Game, Guess, Player, Team};

    use crate::game::tests::rand_game;

    #[test]
    fn join() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game
            .join(Player {
                team: Team::Blue,
                name: "quz".to_string(),
                is_spy_master: false,
            })
            .unwrap();

        assert_eq!(game_clone.players.len() + 1, updated_game.players.len());

        let failed_update = updated_game.join(Player {
            team: Team::Red,
            name: "quz".to_string(),
            is_spy_master: true,
        });

        assert!(failed_update.is_err())
    }

    #[test]
    fn leave() {
        let game = rand_game();
        let game_clone = game.clone();
        let updated_game = game.leave("foo");

        assert_eq!(game_clone.players.len() - 1, updated_game.players.len())
    }

    #[test]
    fn serialize() {
        let game = rand_game();
        let j = serde_json::to_string(&game).unwrap();

        println!("{}", j);
    }

    #[test]
    fn guess() {
        let game: Game = rand_game();
        let game_clone = game.clone();
        let updated_game = game.guess(Guess { board_index: 0 }).unwrap();

        assert_eq!(game_clone.guesses.len() + 1, updated_game.guesses.len());

        let failed_update = updated_game.guess(Guess { board_index: 0 });

        assert!(failed_update.is_err())
    }
}
