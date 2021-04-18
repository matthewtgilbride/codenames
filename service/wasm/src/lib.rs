#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
extern crate wasmcloud_actor_logging as logging;

use guest::prelude::*;
use log::debug;
use wasmcloud_actor_core as core;
use wasmcloud_actor_http_server::{Handlers, Request, Response};

use codenames_domain::game::service::Service;

use crate::dictionary::service::WordGeneratorWasmCloud;
use crate::game::board::service::BoardGeneratorWasmCloud;
use crate::game::dao::WasmKeyValueDao;
use crate::game::game_router::{GameRootRouter, GameRouter};
use crate::root_router::RootRouter;
use crate::routed_request::{RoutedRequest, RoutedRequestHandler};

mod dictionary;
mod game;
mod root_router;
mod routed_request;

#[core::init]
fn init() {
    Handlers::register_handle_request(route_request);
    logging::enable_macros();
}

fn route_request(msg: Request) -> HandlerResult<Response> {
    let word_generator = Box::new(WordGeneratorWasmCloud);
    let board_generator = Box::new(BoardGeneratorWasmCloud);
    let dao = Box::new(WasmKeyValueDao);

    let service = Service::new(word_generator, board_generator, dao)?;

    debug!("Request received. Path is {}", msg.path);

    let root_response = RootRouter::new(&service).handle(&RoutedRequest::new(&msg))?;
    if root_response.is_some() {
        return Ok(root_response.unwrap());
    }

    let game_request = RoutedRequest::new(&msg).pop()?;
    let game_root_response = GameRootRouter::new(&service).handle(&game_request)?;
    if game_root_response.is_some() {
        return Ok(game_root_response.unwrap());
    }

    let game_id_request = game_request.pop()?;
    if game_id_request.clone().path_head.is_none() {
        return Ok(Response::not_found());
    }

    let game_response = GameRouter::new(&service, game_id_request.clone().path_head.unwrap())
        .handle(&game_id_request)?;

    if game_response.is_some() {
        return Ok(game_response.unwrap());
    }

    Ok(Response::not_found())
}
