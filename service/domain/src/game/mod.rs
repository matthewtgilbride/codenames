pub mod board_service;
pub mod dao;
pub mod model;
pub mod service;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::game::model::{Card, CardColor, GameData, GuessRequest, Player, Team};

    fn test_game() -> GameData {
        let cards: Vec<Card> = (0..25)
            .into_iter()
            .map(|i| {
                let color: CardColor = match i {
                    blue if blue < 9 => CardColor::Team(Team::Blue),
                    red if red < 17 => CardColor::Team(Team::Red),
                    death if death < 18 => CardColor::Death,
                    _ => CardColor::Neutral,
                };
                Card {
                    color,
                    word: i.to_string(),
                }
            })
            .collect();

        let game = GameData::new("test".to_string(), cards.try_into().unwrap(), Team::Blue);

        let players: Vec<Player> = vec![
            Player {
                team: Team::Blue,
                name: "foo".to_string(),
                spymaster_secret: Some("".into()),
            },
            Player {
                team: Team::Blue,
                name: "bar".to_string(),
                spymaster_secret: None,
            },
            Player {
                team: Team::Red,
                name: "baz".to_string(),
                spymaster_secret: Some("".into()),
            },
            Player {
                team: Team::Red,
                name: "buzz".to_string(),
                spymaster_secret: None,
            },
        ];

        players
            .iter()
            .fold(game, |game, p| game.join(p.clone()).unwrap())
    }

    #[test]
    fn join() {
        let game = test_game();
        let game_clone = game.clone();
        let updated_game = game
            .join(Player {
                team: Team::Blue,
                name: "quz".to_string(),
                spymaster_secret: None,
            })
            .unwrap();

        assert_eq!(
            game_clone.info.players().len() + 1,
            updated_game.info.players().len()
        );

        let failed_update = updated_game.join(Player {
            team: Team::Red,
            name: "quz".to_string(),
            spymaster_secret: Some("".into()),
        });

        assert!(failed_update.is_err())
    }

    #[test]
    fn leave() {
        let game = test_game();
        let game_clone = game.clone();
        let updated_game = game.leave("foo").unwrap();

        assert_eq!(
            game_clone.info.players().len() - 1,
            updated_game.info.players().len()
        )
    }

    #[test]
    fn serialize() {
        let game = test_game();
        let j = serde_json::to_string(&game).unwrap();

        println!("{}", j);
    }

    #[test]
    fn guess() {
        let game: GameData = test_game();
        let game_clone = game.clone();

        let player_name = game
            .info
            .players()
            .iter()
            .find(|&p| &p.team == game.info.current_turn().team() && p.spymaster_secret.is_none())
            .cloned()
            .unwrap()
            .clone()
            .name;

        let updated_game = game.guess((player_name.as_str(), 0)).unwrap();

        assert_eq!(
            game_clone.info.guesses().len() + 1,
            updated_game.info.guesses().len()
        );

        let failed_update = updated_game.guess((player_name.as_str(), 0));

        assert!(failed_update.is_err())
    }
}
