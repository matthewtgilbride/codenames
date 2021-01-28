use crate::game::api::{generate_board, generate_board_words, generate_game_name, get_dictionary};
use crate::game::dao::DAO;
use crate::game::model::{DictionaryType, Game, NewGameRequest, StandardResult};

pub struct Service {
    pub dao: Box<dyn DAO>,
}

impl Service {
    pub fn random_name() -> StandardResult<NewGameRequest> {
        let dict = get_dictionary(DictionaryType::Default)?;
        let name = generate_game_name(dict)?;
        Ok(NewGameRequest { name })
    }

    pub fn new(request: NewGameRequest) -> StandardResult<Game> {
        let dict = get_dictionary(DictionaryType::Default)?;
        let words = generate_board_words(dict)?;
        let (board, first_team) = generate_board(words)?;

        Ok(Game::new(request.name, board, first_team)?)
    }

    pub fn get(&mut self, key: String) -> StandardResult<Game> {
        self.dao.get(key)
    }

    pub fn save(&mut self, key: String, game: Game) -> StandardResult<()> {
        self.dao.set(key, game)
    }
}
