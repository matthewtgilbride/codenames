#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;

use actor_core as core;
use actor_http_server as http;
use guest::prelude::*;

use crate::dictionary::service::{Service as DictionaryService, WordGeneratorRand};
use crate::game::board::service::{BoardGeneratorRand, Service as BoardService};
use crate::game::dao::RedisDao;
use crate::game::routes::Routes;
use crate::game::service::Service;

mod dictionary;
mod game;
mod model;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(route_wrapper);
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    let word_generator = Box::new(WordGeneratorRand {});
    let dictionary_service = DictionaryService::new(word_generator)?;

    let game_generator = Box::new(BoardGeneratorRand {});
    let board_service = BoardService::new(game_generator);

    let dao = Box::new(RedisDao::new()?);

    let service = Service::new(board_service, dictionary_service, dao);

    let mut routes = Routes::new(service);

    if msg.path == "/" {
        return routes.random_name(msg);
    }
    if msg.path.starts_with("/game") {
        if msg.method == "GET" {
            return routes.get(msg);
        }
        if msg.method == "POST" {
            return routes.new_game(msg);
        }
        if msg.method == "PUT" {
            if msg.path.ends_with("/join") {
                return routes.join(msg);
            }
            if msg.path.ends_with("/guess") {
                return routes.guess(msg);
            }
            if msg.path.ends_with("/end-turn") {
                return routes.end_turn(msg);
            }
        }
    }
    Ok(http::Response::not_found())
}
