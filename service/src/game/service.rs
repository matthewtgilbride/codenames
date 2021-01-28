use crate::game::api::{generate_board, generate_board_words, generate_game_name, get_dictionary};
use crate::game::dao::DAO;
use crate::game::model::{DictionaryType, Game, NewGameRequest, StandardResult};

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

pub fn get(mut dao: Box<dyn DAO>, key: String) -> StandardResult<Game> {
    dao.get(key)
}

pub fn save(mut dao: Box<dyn DAO>, key: String, game: Game) -> StandardResult<()> {
    dao.set(key, game)
}
