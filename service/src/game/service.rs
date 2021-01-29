use crate::dictionary::model::DictionaryType;
use crate::game::api::{generate_board, generate_board_words, generate_game_name};
use crate::game::dao::DAO;
use crate::game::model::{Game, NewGameRequest};
use crate::model::StandardResult;
use crate::game::board::service::Service as GameService;
use crate::dictionary::util::get_dictionary_words;
use crate::dictionary::service::Service as DictionaryService;

pub struct Service {
    game_service: GameService,
    dictionary_service: DictionaryService,
    dao: Box<dyn DAO>,
}

impl Service {
    pub fn new(game_service: GameService, dictionary_service: DictionaryService, dao: Box<dyn DAO>) -> Service {
        Service { game_service, dictionary_service, dao }
    }

    pub fn random_name(&self) -> StandardResult<NewGameRequest> {
        let (first_name, last_name) = self.dictionary_service.new_word_pair()?;
        Ok(NewGameRequest { name: format!("{}-{}", first_name, last_name) })
    }

    pub fn new_game(&self, request: NewGameRequest) -> StandardResult<Game> {
        let words = self.dictionary_service.new_word_set()?;
        let (board, first_team) = self.game_service.new_board(words)?;

        Ok(Game::new(request.name, board, first_team)?)
    }

    pub fn get(&mut self, key: String) -> StandardResult<Game> {
        self.dao.get(key)
    }

    pub fn save(&mut self, key: String, game: Game) -> StandardResult<()> {
        self.dao.set(key, game)
    }
}
