use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::HandlerResult;
use codenames_domain::game::model::{GameVariant, NewGameRequest};
use codenames_domain::game::service::Service;
use codenames_domain::{ServiceError, ServiceResult};
use serde_json::ser::State::Rest;
use std::net::Shutdown::Read;
use wasmcloud_actor_http_server::{Request, Response};

pub struct GameRootRouter {
    service: Service,
}

impl GameRootRouter {
    pub fn new(service: &Service) -> Self {
        Self {
            service: service.clone(),
        }
    }

    fn new_game(&self, msg: Request) -> HandlerResult<Response> {
        let body: NewGameRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let game = self.service.new_game(body)?;
        Ok(Response::json(game, 200, "OK"))
    }
}

impl RoutedRequestHandler for GameRootRouter {
    fn handle(&self, request: RoutedRequest) -> Option<HandlerResult<Response>> {
        match (request.path_head, request.path_tail.len()) {
            (Some(s) , 0) if s.as_str() == "game" => match request.original_request.method.as_str() {
                "POST" => Some(self.clone().new_game(request.original_request)),
                _ => None,
            },
            (Some(_), _) | (None, _) => None,
        }
    }
}

pub struct GameRouter {
    service: Service,
    game_id: String,
}

impl GameRouter {
    pub fn new(service: &Service, game_id: String) -> Self {
        Self {
            game_id,
            service: service.clone(),
        }
    }
}

impl RoutedRequestHandler for GameRouter {
    fn handle(&self, request: RoutedRequest) -> Option<HandlerResult<Response>> {
        match (
            request.path_tail.len(),
            request.original_request.method.as_str(),
        ) {
            (0, "GET") => {
                let get_result = self.service.clone().get(self.game_id.clone(), None);
                match get_result {
                    Ok(game) => Some(Ok(Response::json(game, 200, "OK"))),
                    Err(e) => match e {
                        ServiceError::NotFound(_) => Some(Ok(Response::not_found())),
                        ServiceError::BadRequest(_) => Some(Ok(Response::bad_request())),
                        ServiceError::Unknown(u) => Some(Ok(Response::internal_server_error(u.as_str())))
                    },
                }
            },
            _ => None,
        }
    }
}
