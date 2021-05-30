#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
extern crate wasmcloud_actor_logging as logging;

use guest::prelude::*;
use log::debug;
use urlencoding::decode;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server::{Handlers, Method, Request, Response};

use codenames_domain::game::model::{GuessRequest, NewGameRequest, Player, PlayerRequest};
use codenames_domain::game::service::GameService;
use codenames_domain::ServiceError;

use crate::dictionary::WordGeneratorWasmCloud;
use crate::game::board::BoardGeneratorWasmCloud;
use crate::game::dao::WasmKeyValueDao;

mod dictionary;
mod game;

#[core::init]
fn init() {
    Handlers::register_handle_request(route_request);
    logging::enable_macros();
}

fn route_request(req: Request) -> HandlerResult<Response> {
    let word_generator = Box::new(WordGeneratorWasmCloud);
    let board_generator = Box::new(BoardGeneratorWasmCloud);
    let dao = Box::new(WasmKeyValueDao);
    let service = GameService::new(word_generator, board_generator, dao)?;

    debug!("Request received: Path is {}", req.path);

    let method = req.method();
    let segments = req.path_segments();

    let routing_result: Result<Response, ServiceError> = match (method.clone(), &segments[..]) {
        // get a random game key
        (Method::Get, [""]) => {
            debug_route("random game");
            let json = json!(service.random_name()?);
            Ok(Response::json(json, 200, "OK"))
        }

        // create a game
        (Method::Post, ["game"]) => {
            debug_route("create game");
            let body: NewGameRequest =
                serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
            let game = service.new_game(body)?;
            Ok(Response::json(game, 200, "OK"))
        }

        // get a list of all games
        (Method::Get, ["game"]) => {
            debug_route("get all games");
            let games = service.clone().find()?;
            Ok(Response::json(games, 200, "OK"))
        }

        // get an existing game
        (Method::Get, ["game", game_key]) => {
            debug_route("get game");
            let game = service.clone().get(game_key.to_string(), None)?;
            Ok(Response::json(game, 200, "OK"))
        }

        // join a game as a player
        (Method::Put, ["game", game_key, "join"]) => {
            debug_route("join");
            let player: Player = serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
            let updated_game = service.join(game_key.to_string(), player)?;
            Ok(Response::json(updated_game, 200, "OK"))
        }

        // undo a guess
        (Method::Put, ["game", game_key, "guess", "undo"]) => {
            debug_route("undo guess");
            let updated_game = service.undo_guess(game_key.to_string())?;
            Ok(Response::json(updated_game, 200, "OK"))
        }

        // end the current team's turn
        (Method::Put, ["game", game_key, "end-turn"]) => {
            debug_route("end turn");
            let updated_game = service.end_turn(game_key.to_string())?;
            Ok(Response::json(updated_game, 200, "OK"))
        }

        // get a player's view of the game
        (Method::Get, ["game", game_key, player_name_encoded]) => {
            debug_route("get player game");
            let player_name = decode(player_name_encoded)?;
            let game = service.clone().get(
                game_key.to_string(),
                Some(PlayerRequest::new(player_name.as_str())),
            )?;
            Ok(Response::json(game, 200, "OK"))
        }

        // guess a word
        (Method::Put, ["game", game_key, player_name_encoded, "guess", index]) => {
            debug_route("guess");
            let player_name = decode(player_name_encoded)?;
            let board_index_result = index.parse::<usize>();
            match board_index_result {
                Ok(board_index) => {
                    let updated_game = service.guess(
                        game_key.to_string(),
                        GuessRequest::new(player_name.as_str(), board_index),
                    )?;
                    Ok(Response::json(updated_game, 200, "OK"))
                }
                Err(e) => Err(ServiceError::BadRequest(e.to_string())),
            }
        }

        // leave a game
        (Method::Put, ["game", game_key, player_name_encoded, "leave"]) => {
            debug_route("leave");
            let player_name = decode(player_name_encoded)?;
            let updated_game = service.clone().leave(
                game_key.to_string(),
                PlayerRequest::new(player_name.as_str()),
            )?;
            Ok(Response::json(updated_game, 200, "OK"))
        }

        _ => Ok(Response::not_found()),
    };

    Ok(routing_result.unwrap_or_else(|se| {
        Response::json(
            se.clone(),
            match se {
                ServiceError::BadRequest(_) => 400,
                ServiceError::NotFound(_) => 404,
                ServiceError::Unknown(_) => 500,
            },
            "ServiceError",
        )
    }))
}

fn debug_route(msg: &str) {
    debug!("matched route: {}", msg)
}
