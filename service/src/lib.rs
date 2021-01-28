#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;

use actor_core as core;
use actor_http_server as http;
use guest::prelude::*;

use crate::game::dao::RedisDao;
use crate::game::routes::Routes;
use crate::game::service::Service;

mod game;

#[no_mangle]
pub fn wapc_init() {
    core::Handlers::register_health_request(health);
    http::Handlers::register_handle_request(route_wrapper);
}

fn health(_h: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
    Ok(core::HealthCheckResponse::healthy())
}

fn route_wrapper(msg: http::Request) -> HandlerResult<http::Response> {
    let dao = Box::new(RedisDao::new()?);
    let service = Service { dao };
    let routes = Routes { service };
    if msg.path == "/" {
        return routes.random_name(msg);
    }
    if msg.path.starts_with("/game") {
        if msg.method == "GET" {
            return Routes::get(msg);
        }
        if msg.method == "POST" {
            return routes.new(msg);
        }
        if msg.method == "PUT" {
            if msg.path.ends_with("/join") {
                return Routes::join(msg);
            }
            if msg.path.ends_with("/guess") {
                return Routes::guess(msg);
            }
            if msg.path.ends_with("/end-turn") {
                return Routes::end_turn(msg);
            }
        }
    }
    Ok(http::Response::not_found())
}
