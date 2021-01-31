pub mod board;
pub mod dao;
pub mod model;

#[cfg(test)]
mod tests {
    use codenames_domain::game::dao::DAO;
    use codenames_domain::game::model::{Game, Player, Team};
    use codenames_domain::game::service::Service;
    use codenames_domain::StdResult;

    use crate::dictionary::service::WordGeneratorRand;
    use crate::game::board::service::BoardGeneratorRand;

    struct DaoStub;

    impl DAO for DaoStub {
        fn get(&mut self, _: String) -> StdResult<Game> {
            unimplemented!()
        }

        fn set(&mut self, _: String, _: Game) -> StdResult<()> {
            unimplemented!()
        }
    }

    pub fn rand_game() -> Game {
        let word_generator = Box::new(WordGeneratorRand {});
        let board_generator = Box::new(BoardGeneratorRand {});
        let dao = Box::new(DaoStub {});

        let test_game_service = Service::new(word_generator, board_generator, dao).unwrap();

        let game = test_game_service
            .new_game(test_game_service.random_name().unwrap())
            .unwrap();

        let players: Vec<Player> = vec![
            Player {
                team: Team::Blue,
                name: "foo".to_string(),
                is_spy_master: true,
            },
            Player {
                team: Team::Blue,
                name: "bar".to_string(),
                is_spy_master: false,
            },
            Player {
                team: Team::Red,
                name: "baz".to_string(),
                is_spy_master: true,
            },
            Player {
                team: Team::Red,
                name: "buzz".to_string(),
                is_spy_master: false,
            },
        ];

        players
            .iter()
            .fold(game, |game, p| game.join(p.clone()).unwrap())
    }
}
