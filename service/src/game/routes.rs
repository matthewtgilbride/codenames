extern crate wapc_guest as guest;

use crate::game;
use crate::game::dictionary::DictionaryType;

use crate::game::model::{Player, Team, Game};
use actor_http_server as http;
use actor_keyvalue as kv;
use guest::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct NewGameRequest {
    name: String,
}

pub fn new_game(msg: http::Request) -> HandlerResult<http::Response> {
    let body: NewGameRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;

    let dict = game::dictionary::get_dictionary(DictionaryType::Default)?;
    let words = game::api::generate_board_words(dict)?;
    let (board, first_team) = game::api::generate_board(words)?;

    let game = game::model::Game::new(body.name, board, first_team, Vec::new())?;
    let json = json!(game);

    let key = Uuid::new_v4().to_string();
    let _ = kv::default().set(key, json.to_string(), 0)?;

    Ok(http::Response::json(json, 200, "OK"))
}

pub fn join_game(msg: http::Request) -> HandlerResult<http::Response> {
    let player: Player = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;

    let game_key: Option<String> = msg
        .path
        .split("/")
        .into_iter()
        .map(|s| s.to_string())
        .last();

    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |k| {
            let game_json = kv::default().get(k.clone())?;

            if !game_json.exists { return Ok(http::Response::not_found()); }

            let game: Game = serde_json::from_str(game_json.value.as_str())?;

            let updated_game = game.join(player)?;
            let json = json!(updated_game);
            let _ = kv::default().set(k, json.to_string(), 0);

            Ok(http::Response::ok())
        }
    )
}
