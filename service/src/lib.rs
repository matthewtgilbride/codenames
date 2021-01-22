mod game;

#[macro_use]
extern crate serde_json;

extern crate wapc_guest as guest;
use actor_core as core;
use actor_http_server as http;
use actor_keyvalue as kv;
use guest::prelude::*;

use crate::game::dictionary::{get_dictionary, DictionaryType};

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(route_wrapper);
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    if msg.path.starts_with("/game") {
        if msg.method == "POST" {
            return game::routes::new_game(msg);
        } else if msg.method == "PUT" {
            if msg.path.contains("/join/") {
                return game::routes::join_game(msg);
            }
        }
    }
    Ok(http::Response::not_found())
}
