use crate::game::api::{generate_board, generate_board_words, generate_game_name, get_dictionary};
use crate::game::dao;
use crate::game::dao::{RedisDao, DAO};
use crate::game::model::{DictionaryType, Game, NewGameRequest};
use std::error::Error;
use std::str::Utf8Error;

pub fn random_name() -> Result<NewGameRequest, String> {
    let dict = get_dictionary(DictionaryType::Default).map_err(|e| e.to_string())?;
    let name = generate_game_name(dict).map_err(|e| e.to_string())?;
    Ok(NewGameRequest { name })
}

pub fn new(request: NewGameRequest) -> Result<Game, String> {
    let dict = get_dictionary(DictionaryType::Default).map_err(|e| e.to_string())?;
    let words = generate_board_words(dict).map_err(|e| e.to_string())?;
    let (board, first_team) = generate_board(words)?;

    Ok(Game::new(request.name, board, first_team)?)
}

pub fn get(mut dao: Box<dyn DAO>, key: String) -> Result<Game, Box<dyn Error>> {
    dao.get(key)
}

pub fn save(mut dao: Box<dyn DAO>, key: String, game: Game) -> Result<(), Box<dyn Error>> {
    dao.set(key, game)
}
