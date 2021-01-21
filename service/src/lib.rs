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
    if msg.path.starts_with("/game/new") {
        return new_game(msg)
    }
    Ok(http::Response::not_found())
}

fn new_game(msg: http::Request) -> HandlerResult<http::Response> {
    let dict = game::dictionary::get_dictionary(DictionaryType::Default)?;
    let words = game::api::generate_board_words(dict)?;
    let (board, first_team) = game::api::generate_board(words)?;
    let key = "test".to_string();
    let game = game::model::Game::new(key, board, first_team, Vec::new())?;
    let json = json!(game);
    // let _ = kv::default().add(key.clone(), json!(game))?; TODO: put json in kv store?
    Ok(http::Response::json(json, 200, "OK"))
}
