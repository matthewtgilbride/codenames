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

    match RootRouter::new(&service).handle(&RoutedRequest::new(&msg))? {
        Some(r) => Ok(r),
        None => Ok(Response::not_found()),
    }
}
