#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;

use guest::prelude::*;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server as http;

use codenames_domain::game::service::Service;

use crate::dictionary::service::WordGeneratorWasmCloud;
use crate::game::board::service::BoardGeneratorWasmCloud;
use crate::game::dao::WasmKeyValueDao;
use crate::wasm_routes::WasmRoutes;

// use wasm_routes::WasmRoutes;

mod dictionary;
mod game;
mod wasm_routes;

#[core::init]
fn init() {
 http::Handlers::register_handle_request(route_wrapper);
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    let word_generator = Box::new(WordGeneratorWasmCloud);
    let board_generator = Box::new(BoardGeneratorWasmCloud);
    let dao = Box::new(WasmKeyValueDao);

    let service = Service::new(word_generator, board_generator, dao)?;

    let mut routes = WasmRoutes::new(service);

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
            if msg.path.ends_with("/leave") {
                return routes.leave(msg);
            }
            if msg.path.ends_with("/guess") {
                return routes.guess(msg);
            }
            if msg.path.ends_with("/guess/undo") {
                return routes.undo_guess(msg);
            }
            if msg.path.ends_with("/end-turn") {
                return routes.end_turn(msg);
            }
        }
    }
    Ok(http::Response::not_found())
}
