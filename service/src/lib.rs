use actor_core as core;
use actor_http_server as http;
use actor_keyvalue as kv;
use guest::prelude::*;

mod game;
use game::routes as game_routes;

#[macro_use]
extern crate serde_json;

extern crate wapc_guest as guest;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(route_wrapper);
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    if msg.path == "/" {
        return game_routes::random_name(msg);
    }
    if msg.path.starts_with("/game") {
        if msg.method == "GET" {
            return game_routes::get(msg);
        }
        if msg.method == "POST" {
            return game_routes::new(msg);
        }
        if msg.method == "PUT" {
            if msg.path.ends_with("/join") {
                return game_routes::join(msg);
            }
            if msg.path.ends_with("/guess") {
                return game_routes::guess(msg);
            }
            if msg.path.ends_with("/end-turn") {
                return game_routes::end_turn(msg);
            }
        }
    }
    Ok(http::Response::not_found())
}
