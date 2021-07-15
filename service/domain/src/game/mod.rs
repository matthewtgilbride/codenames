pub mod board;
pub mod card;
pub mod dao;
pub mod model;
pub mod service;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::game::card::{Card, CardColor};
    use crate::game::model::{GameData, GuessRequest, Player, Team};

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

        assert_eq!(game_clone.players.len() + 1, updated_game.players.len());

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

        assert_eq!(game_clone.players.len() - 1, updated_game.players.len())
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
            .players
            .iter()
            .find(|(_, p)| p.team == game.turn && p.spymaster_secret.is_none())
            .map(|(_, p)| p)
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
