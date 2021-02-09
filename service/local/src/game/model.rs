#[cfg(test)]
mod tests {
    use codenames_domain::game::model::{Game, GuessRequest, Team};

    use crate::game::tests::rand_game;
    use codenames_domain::game::player::model::Player;

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

        let player_name = game
            .players
            .iter()
            .find(|p| p.team == game.turn)
            .unwrap()
            .clone()
            .name;
        let updated_game = game
            .guess(GuessRequest {
                board_index: 0,
                player_name: player_name.clone(),
            })
            .unwrap();

        assert_eq!(game_clone.guesses.len() + 1, updated_game.guesses.len());

        let failed_update = updated_game.guess(GuessRequest {
            board_index: 0,
            player_name: player_name.clone(),
        });

        assert!(failed_update.is_err())
    }
}
