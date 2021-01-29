use crate::dictionary::service::Service as DictionaryService;
use crate::game::board::service::Service as BoardService;
use crate::game::dao::DAO;
use crate::game::model::{Game, NewGameRequest};
use crate::model::StandardResult;

pub struct Service {
    board_service: BoardService,
    dictionary_service: DictionaryService,
    dao: Box<dyn DAO>,
}

impl Service {
    pub fn new(
        board_service: BoardService,
        dictionary_service: DictionaryService,
        dao: Box<dyn DAO>,
    ) -> Service {
        Service {
            board_service,
            dictionary_service,
            dao,
        }
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

    pub fn get(&mut self, key: String) -> StandardResult<Game> {
        self.dao.get(key)
    }

    pub fn save(&mut self, key: String, game: Game) -> StandardResult<()> {
        self.dao.set(key, game)
    }
}
