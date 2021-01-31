#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;

use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use actor_core as core;
use actor_http_server as http;
use guest::prelude::*;

use codenames_domain::dictionary::service::WordGenerator;
use codenames_domain::game::board::service::BoardGenerator;
use codenames_domain::game::card::model::Card;
use codenames_domain::game::dao::DAO;
use codenames_domain::game::model::{Game, Team};
use codenames_domain::game::service::Service;
use codenames_domain::StdResult;

use crate::wasm_routes::WasmRoutes;

// use wasm_routes::WasmRoutes;

mod wasm_routes;

struct WordStub;

impl WordGenerator for WordStub {
    fn random_set(&self, _: HashSet<String, RandomState>) -> StdResult<[String; 25]> {
        unimplemented!()
    }

    fn random_pair(&self, _: HashSet<String, RandomState>) -> StdResult<(String, String)> {
        unimplemented!()
    }
}

struct BoardStub;

impl BoardGenerator for BoardStub {
    fn random_board(&self, _: [String; 25]) -> StdResult<([Card; 25], Team)> {
        unimplemented!()
    }
}

struct DaoStub;

impl DAO for DaoStub {
    fn get(&mut self, _: String) -> StdResult<Game> {
        unimplemented!()
    }

    fn set(&mut self, _: String, _: Game) -> StdResult<()> {
        unimplemented!()
    }
}

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(route_wrapper);
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    let word_generator = Box::new(WordStub);
    let board_generator = Box::new(BoardStub);
    let dao = Box::new(DaoStub);

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
