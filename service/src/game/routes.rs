extern crate wapc_guest as guest;

use crate::game::api;
use crate::game::model::{DictionaryType, Game, Guess, Player, Team};
use actor_http_server as http;
use actor_keyvalue as kv;
use guest::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct NewGameRequest {
    name: String,
}

pub fn random_name(msg: http::Request) -> HandlerResult<http::Response> {
    let dict = api::get_dictionary(DictionaryType::Default)?;
    let name = api::generate_game_name(dict)?;
    let json = json!(NewGameRequest { name });
    Ok(http::Response::json(json, 200, "OK"))
}

pub fn new(msg: http::Request) -> HandlerResult<http::Response> {
    let body: NewGameRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;

    let dict = api::get_dictionary(DictionaryType::Default)?;
    let words = api::generate_board_words(dict)?;
    let (board, first_team) = api::generate_board(words)?;

    let game = Game::new(body.name, board, first_team)?;
    let json = json!(game);

    let key = Uuid::new_v4().to_string();
    let _ = kv::default().set(key, json.to_string(), 0)?;

    Ok(http::Response::json(json, 200, "OK"))
}

pub fn get(msg: http::Request) -> HandlerResult<http::Response> {
    let game_key = get_game_key(msg.path);
    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |k| {
            let game_json = kv::default().get(k.clone())?;

            if !game_json.exists {
                return Ok(http::Response::not_found());
            }

            let game: Game = serde_json::from_str(game_json.value.as_str())?;
            let json = json!(game);

            Ok(http::Response::json(game, 200, "OK"))
        },
    )
}

pub fn join(msg: http::Request) -> HandlerResult<http::Response> {
    let player: Player = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;

    let game_key = get_game_key(msg.path);

    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |k| {
            let game_json = kv::default().get(k.clone())?;

            if !game_json.exists {
                return Ok(http::Response::not_found());
            }

            let game: Game = serde_json::from_str(game_json.value.as_str())?;

            let updated_game = game.join(player)?;
            let json = json!(updated_game);
            let _ = kv::default().set(k, json.to_string(), 0);

            Ok(http::Response::ok())
        },
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct GuessRequest {
    player_name: String,
    board_index: usize,
}

pub fn guess(msg: http::Request) -> HandlerResult<http::Response> {
    let guess: GuessRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;

    let game_key = get_game_key(msg.path);

    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |k| {
            let game_json = kv::default().get(k.clone())?;

            if !game_json.exists {
                return Ok(http::Response::not_found());
            }

            let game: Game = serde_json::from_str(game_json.value.as_str())?;

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
                    || Ok(http::Response::bad_request()),
                    |player| {
                        let updated_game = game.clone().guess(Guess {
                            team: game.turn,
                            board_index: guess.board_index,
                        });
                        let json = json!(updated_game);
                        let _ = kv::default().set(k, json.to_string(), 0);

                        Ok(http::Response::ok())
                    },
                )
        },
    )
}

pub fn end_turn(msg: http::Request) -> HandlerResult<http::Response> {
    let game_key = get_game_key(msg.path);
    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |k| {
            let game_json = kv::default().get(k.clone())?;

            if !game_json.exists {
                return Ok(http::Response::not_found());
            }

            let game: Game = serde_json::from_str(game_json.value.as_str())?;

            let updated_game = game.end_turn();
            let json = json!(updated_game);
            let _ = kv::default().set(k, json.to_string(), 0);

            Ok(http::Response::ok())
        },
    )
}

fn get_game_key(path: String) -> Option<String> {
    path.split("/")
        .into_iter()
        .map(|s| s.to_string())
        .find(|path_part| {
            vec!["game", "join"]
                .iter()
                .cloned()
                .find(|s| s == path_part)
                .is_none()
        })
}
