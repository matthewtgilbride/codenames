extern crate wapc_guest as guest;

use crate::game::api;
use crate::game::model::{DictionaryType, Game, Guess, Player, Team};
use actor_http_server as http;
use actor_http_server::Response;
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
    save_and_respond(Uuid::new_v4().to_string(), game, true)
}

pub fn get(msg: http::Request) -> HandlerResult<http::Response> {
    with_game_or_not_found(msg, |_, game| {
        let json = json!(game);
        Ok(http::Response::json(game, 200, "OK"))
    })
}

pub fn join(msg: http::Request) -> HandlerResult<http::Response> {
    let player: Player = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
    with_game_or_not_found(msg, |key, game| {
        let updated_game = game.join(player.clone())?;
        save_and_respond(key, updated_game, false)
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct GuessRequest {
    player_name: String,
    board_index: usize,
}

pub fn guess(msg: http::Request) -> HandlerResult<http::Response> {
    let guess: GuessRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
    with_game_or_not_found(msg, |key, game| {
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
                    })?;
                    save_and_respond(key, updated_game, false)
                },
            )
    })
}

pub fn end_turn(msg: http::Request) -> HandlerResult<http::Response> {
    with_game_or_not_found(msg, |key, game| {
        let updated_game = game.end_turn();
        save_and_respond(key, updated_game, false)
    })
}

fn save_and_respond(key: String, game: Game, with_payload: bool) -> HandlerResult<http::Response> {
    let json = json!(game);
    let _ = kv::default().set(key, json.to_string(), 0);
    Ok(match with_payload {
        true => http::Response::json(game, 200, "OK"),
        false => http::Response::ok(),
    })
}

fn with_game_or_not_found(
    msg: http::Request,
    game_fn: impl Fn(String, Game) -> HandlerResult<http::Response>,
) -> HandlerResult<http::Response> {
    let game_key = get_game_key(msg.path);
    game_key.map_or_else(
        || Ok(http::Response::not_found()),
        |key| {
            kv::default().get(key.clone()).map_or_else(
                |_| Ok(http::Response::not_found()),
                |game_json| {
                    let game: Game = serde_json::from_str(game_json.value.as_str())?;
                    game_fn(key, game)
                },
            )
        },
    )
}

fn get_game_key(path: String) -> Option<String> {
    path.split("/")
        .into_iter()
        .find(|&path_part| !is_path_segment(path_part.to_string()) && path_part.len() > 0)
        .map(|s| s.to_string())
}

fn is_path_segment(part: String) -> bool {
    ["game", "join", "guess", "end-turn"]
        .iter()
        .cloned()
        .find(|s| s == &part)
        .is_some()
}

#[cfg(test)]
mod is_path_segment {
    #[test]
    fn truthy() {
        assert!(super::is_path_segment("game".to_string()))
    }

    #[test]
    fn falsy() {
        assert!(!super::is_path_segment("foo".to_string()))
    }
}

#[cfg(test)]
mod get_game_key {
    #[test]
    fn valid_url() {
        let key = "aaaa-aaaa-aaaa-aaaa";
        let path = format!("/game/{}/join", key);
        let result = super::get_game_key(path);
        assert_eq!(key, result.unwrap())
    }

    #[test]
    fn invalid_url() {
        let path = "/game/join".to_string();
        let result = super::get_game_key(path);
        assert!(result.is_none())
    }
}
