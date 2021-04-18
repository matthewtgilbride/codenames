use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::HandlerResult;
use codenames_domain::game::model::{GameVariant, NewGameRequest, Player, PlayerRequest};
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
        match (
            request.msg.method.as_str(),
            request.path_head,
            request.path_tail.len(),
        ) {
            ("POST", Some(s), 0) if s.as_str() == "game" => {
                Some(self.clone().new_game(request.msg))
            }
            _ => None,
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
            request.msg.method.as_str(),
            request.path_tail.get(0).map(|s| s.as_str()),
            request.path_tail.get(1).map(|s| s.as_str()),
        ) {
            ("GET", None, None) => self
                .service
                .clone()
                .get(self.game_id.clone(), None)
                .map(|game| Some(Ok(Response::json(game, 200, "OK"))))
                .unwrap_or_else(|e| RoutedRequest::handle_service_error(e)),
            ("PUT", Some("join"), None) => std::str::from_utf8(request.msg.body.as_slice())
                .map(|body| {
                    serde_json::from_str(body)
                        .map(|player: Player| {
                            self.service
                                .join(self.game_id.clone(), player)
                                .map(|updated_game| {
                                    Some(Ok(Response::json(updated_game, 200, "OK")))
                                })
                                .unwrap_or_else(|e| RoutedRequest::handle_service_error(e))
                        })
                        .unwrap_or_else(|e| Some(Ok(Response::bad_request())))
                })
                .unwrap_or_else(|e| Some(Ok(Response::internal_server_error("Utf8Error")))),
            ("PUT", Some("leave"), None) => None,
            ("PUT", Some("guess"), None) => None,
            ("PUT", Some("end-turn"), None) => None,
            ("PUT", Some("guess"), Some("undo")) => None,
            _ => None,
        }
    }
}
