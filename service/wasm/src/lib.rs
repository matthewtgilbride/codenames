#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
extern crate wasmcloud_actor_logging as logging;

use guest::prelude::*;
use log::debug;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server::{Handlers, Method, Request, Response};

use codenames_domain::game::service::Service;

use crate::dictionary::service::WordGeneratorWasmCloud;
use crate::game::board::service::BoardGeneratorWasmCloud;
use crate::game::dao::WasmKeyValueDao;
use codenames_domain::game::model::{GuessRequest, NewGameRequest, Player, PlayerRequest};

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
    let service = Service::new(word_generator, board_generator, dao)?;

    debug!("Request received: Path is {}", req.path);

    let method = req.method();
    let segments = req.path_segments();

    match (method.clone(), segments.get(0), segments.get(1)) {

        (Method::Get, None, None) => {
            let json = json!(service.random_name()?);
            Ok(Response::json(json, 200, "OK"))
        }

        (Method::Post, Some(&"game"), None) => {
            let body: NewGameRequest =
                serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
            let game = service.new_game(body)?;
            Ok(Response::json(game, 200, "OK"))
        }

        (_, Some(&"game"), Some(&game)) => {

            let game_id = game.to_string();

            match (method.clone(), segments.get(2), segments.get(3)) {

                (Method::Get, None, None) => {
                    let game = service.clone().get(game_id, None)?;
                    Ok(Response::json(game, 200, "OK"))
                }

                (Method::Put, Some(&"join"), None) => {
                    let player: Player =
                        serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
                    let updated_game = service.join(game_id, player)?;
                    Ok(Response::json(updated_game, 200, "OK"))
                }

                (Method::Put, Some(&"guess"), Some(&"undo")) => {
                    let updated_game = service.undo_guess(game_id)?;
                    Ok(Response::json(updated_game, 200, "OK"))
                }

                (Method::Put, Some(&"end-turn"), None) => {
                    let updated_game = service.end_turn(game_id)?;
                    Ok(Response::json(updated_game, 200, "OK"))
                }

                (_, Some(&player), player_segment) => {

                    let player_name = player.to_string();

                    match (method.clone(), player_segment, segments.get(4)) {

                        (Method::Get, None, None) => {
                            let game = service
                                .clone()
                                .get(game_id.clone(), Some(PlayerRequest { player_name }))?;
                            Ok(Response::json(game, 200, "OK"))
                        }

                        (Method::Put, Some(&"guess"), Some(&index)) => {
                            let board_index_result = index.parse::<usize>();
                            match board_index_result {
                                Ok(board_index) => {
                                    let guess_request = GuessRequest {
                                        player_name,
                                        board_index,
                                    };
                                    let updated_game =
                                        service.guess(game_id.clone(), guess_request)?;
                                    Ok(Response::json(updated_game, 200, "OK"))
                                }
                                Err(_) => Ok(Response::bad_request()),
                            }
                        }

                        (Method::Put, Some(&"leave"), None) => {
                            let updated_game = service
                                .clone()
                                .leave(game_id.clone(), PlayerRequest { player_name })?;
                            Ok(Response::json(updated_game, 200, "OK"))
                        }

                        _ => Ok(Response::not_found()),
                    }
                }
                _ => Ok(Response::not_found()),
            }
        }
        _ => Ok(Response::not_found()),
    }
}
