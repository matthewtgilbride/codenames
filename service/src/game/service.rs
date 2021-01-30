use crate::dictionary::service::{Service as DictionaryService, WordGenerator};
use crate::game::board::service::{BoardGenerator, Service as BoardService};
use crate::game::dao::DAO;
use crate::game::model::{Game, Guess, GuessRequest, NewGameRequest, Player};
use crate::model::StandardResult;

pub struct Service {
    board_service: BoardService,
    dictionary_service: DictionaryService,
    dao: Box<dyn DAO>,
}

impl Service {
    pub fn new(
        word_generator: Box<dyn WordGenerator>,
        board_generator: Box<dyn BoardGenerator>,
        dao: Box<dyn DAO>,
    ) -> StandardResult<Service> {
        let dictionary_service = DictionaryService::new(word_generator)?;
        let board_service = BoardService::new(board_generator);
        Ok(Service {
            board_service,
            dictionary_service,
            dao,
        })
    }

    pub fn random_name(&self) -> StandardResult<NewGameRequest> {
        let (first_name, last_name) = self.dictionary_service.new_word_pair()?;
        Ok(NewGameRequest {
            name: format!("{}-{}", first_name, last_name),
        })
    }

    pub fn new_game(&self, request: NewGameRequest) -> StandardResult<Game> {
        let words = self.dictionary_service.new_word_set()?;
        let (board, first_team) = self.board_service.new_board(words)?;

        Ok(Game::new(request.name, board, first_team)?)
    }

    pub fn guess(guess: GuessRequest, game: Game) -> StandardResult<Game> {
        game.players
            .iter()
            .cloned()
            .find(
                |Player {
                     name,
                     is_spy_master,
                     team,
                     ..
                 }| {
                    *name == guess.player_name && *is_spy_master == false && *team == game.turn
                },
            )
            .map_or_else(
                || Err("guess must be made by a valid player in the game (by name), on the team that matches the game's current turn, who is not a spy master".into()),
                |_|
                    game.clone().guess(Guess {
                        board_index: guess.board_index,
                    }),
            )
    }

    pub fn get(&mut self, key: String) -> StandardResult<Game> {
        self.dao.get(key)
    }

    pub fn save(&mut self, key: String, game: Game) -> StandardResult<()> {
        self.dao.set(key, game)
    }
}
