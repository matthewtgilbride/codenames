use wasmcloud_actor_http_server::{Request, Response};

use codenames_domain::game::model::{NewGameRequest, Player};
use codenames_domain::game::service::Service;

use crate::game::player::player_router::PlayerRouter;
use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::HandlerResult;

pub struct GameRouter {
    service: Service,
}

impl GameRouter {
    pub fn new(service: &Service) -> Self {
        Self {
            service: service.clone(),
        }
    }

    fn new_game(&self, msg: Request) -> HandlerResult<Option<Response>> {
        let body: NewGameRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let game = self.service.new_game(body)?;
        Ok(Some(Response::json(game, 200, "OK")))
    }
}

impl RoutedRequestHandler for GameRouter {
    fn handle(&self, request: &RoutedRequest) -> HandlerResult<Option<Response>> {
        match request.path_head.clone() {
            Some(g) if g.as_str() == "game" => {
                match (request.msg.clone().method.as_str(), request.path_tail.len()) {
                    ("POST", 0) => self.clone().new_game(request.clone().msg),
                    _ => GameIdRouter::new(&self.service).handle(&request.pop()?),
                }
            }
            _ => Ok(None),
        }
    }
}

pub struct GameIdRouter {
    service: Service,
}

impl GameIdRouter {
    pub fn new(service: &Service) -> Self {
        Self {
            service: service.clone(),
        }
    }
}

impl RoutedRequestHandler for GameIdRouter {
    fn handle(&self, request: &RoutedRequest) -> HandlerResult<Option<Response>> {
        match request.path_head.clone() {
            None => Ok(None),
            Some(game_key) => {
                match (
                    request.msg.method.as_str(),
                    request.path_tail.get(0).map(|s| s.as_str()),
                    request.path_tail.get(1).map(|s| s.as_str()),
                ) {
                    ("GET", None, None) => {
                        let game = self.service.clone().get(game_key, None)?;
                        Ok(Some(Response::json(game, 200, "OK")))
                    }
                    ("PUT", Some("join"), None) => {
                        let player: Player = serde_json::from_str(std::str::from_utf8(
                            request.msg.body.as_slice(),
                        )?)?;
                        let updated_game = self.service.join(game_key, player)?;
                        Ok(Some(Response::json(updated_game, 200, "OK")))
                    }
                    ("PUT", Some("guess"), Some("undo")) => {
                        let updated_game = self.service.undo_guess(game_key)?;
                        Ok(Some(Response::json(updated_game, 200, "OK")))
                    }
                    ("PUT", Some("end-turn"), None) => {
                        let updated_game = self.service.end_turn(game_key)?;
                        Ok(Some(Response::json(updated_game, 200, "OK")))
                    }
                    (_, Some(_), _) => {
                        PlayerRouter::new(&self.service, &game_key).handle(&request.clone().pop()?)
                    }
                    _ => Ok(None),
                }
            }
        }
    }
}
