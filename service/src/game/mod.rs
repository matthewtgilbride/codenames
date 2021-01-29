pub mod board;
mod card;
pub mod dao;
mod model;
pub mod routes;
pub mod service;

#[cfg(test)]
mod tests {
    use crate::dictionary::service::{Service as DictionaryService, WordGeneratorRand};
    use crate::game::board::service::{BoardGeneratorRand, Service as BoardService};
    use crate::game::dao::DAO;
    use crate::game::model::{Game, Player, Team};
    use crate::game::service::Service;
    use crate::model::StandardResult;

    struct DaoStub {}

    impl DAO for DaoStub {
        fn get(&mut self, _: String) -> StandardResult<Game> {
            unimplemented!()
        }

        fn set(&mut self, _: String, _: Game) -> StandardResult<()> {
            unimplemented!()
        }
    }

    pub fn rand_game() -> Game {
        let word_generator = Box::new(WordGeneratorRand {});
        let dictionary_service = DictionaryService::new(word_generator).unwrap();

        let game_generator = Box::new(BoardGeneratorRand {});
        let board_service = BoardService::new(game_generator);

        let dao = Box::new(DaoStub {});

        let test_game_service = Service::new(board_service, dictionary_service, dao);

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
