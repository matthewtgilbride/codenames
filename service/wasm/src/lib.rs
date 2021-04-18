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
use crate::game::router::{GameRootRouter, GameRouter};
use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::router::RootRouter;

mod dictionary;
mod game;
mod routed_request;
mod router;

#[core::init]
fn init() {
    Handlers::register_handle_request(route_wrapper);
    logging::enable_macros();
}

fn route_wrapper(msg: Request) -> HandlerResult<Response> {
    let word_generator = Box::new(WordGeneratorWasmCloud);
    let board_generator = Box::new(BoardGeneratorWasmCloud);
    let dao = Box::new(WasmKeyValueDao);

    let service = Service::new(word_generator, board_generator, dao)?;

    let root_router = RootRouter::new(&service);

    debug!("Request received. Path is {}", msg.path);

    let root_request = RoutedRequest::new(&msg);
    let root_response = root_router.handle(root_request.clone())?;
    if root_response.is_some() {
        return Ok(root_response.unwrap());
    }

    let game_request = root_request.pop()?;
    let game_root_router = GameRootRouter::new(&service);
    let game_root_response = game_root_router.handle(game_request.clone())?;
    if game_root_response.is_some() {
        return Ok(game_root_response.unwrap());
    }

    let game_id_request = game_request.pop()?;
    if game_id_request.clone().path_head.is_none() {
        return Ok(Response::not_found());
    }

    let game_router = GameRouter::new(&service, game_id_request.clone().path_head.unwrap());
    let game_response = game_router.handle(game_id_request.clone())?;

    if game_response.is_some() {
        return Ok(game_response.unwrap());
    }

    Ok(Response::not_found())
}
