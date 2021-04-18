use wasmcloud_actor_http_server::{Request, Response};

use codenames_domain::game::model::{NewGameRequest, Player, PlayerRequest, GuessRequest};
use codenames_domain::game::service::Service;

use crate::HandlerResult;
use crate::routed_request::{RoutedRequest, RoutedRequestHandler};

pub struct GameRootRouter {
    service: Service,
}

impl GameRootRouter {
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

impl RoutedRequestHandler for GameRootRouter {
    fn handle(&self, request: RoutedRequest) -> HandlerResult<Option<Response>> {
        match (
            request.msg.method.as_str(),
            request.path_head,
            request.path_tail.len(),
        ) {
            ("POST", Some(s), 0) if s.as_str() == "game" => self.clone().new_game(request.msg),
            _ => Ok(None),
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
    fn handle(&self, request: RoutedRequest) -> HandlerResult<Option<Response>> {
        match (
            request.msg.method.as_str(),
            request.path_tail.get(0).map(|s| s.as_str()),
            request.path_tail.get(1).map(|s| s.as_str()),
        ) {
            ("GET", None, None) => {
                let game = self
                    .service
                    .clone()
                    .get(self.game_id.clone(), None)?;
                Ok(Some(Response::json(game, 200, "OK")))
            },
            ("PUT", Some("join"), None) => {
                let player: Player = serde_json::from_str(std::str::from_utf8(request.msg.body.as_slice())?)?;
                let updated_game = self.service.join(self.game_id.clone(), player)?;
                Ok(Some(Response::json(updated_game, 200, "OK")))
            },
            ("PUT", Some("leave"), None) => {
                let req: PlayerRequest = serde_json::from_str(std::str::from_utf8(request.msg.body.as_slice())?)?;
                let updated_game = self.service.leave(self.game_id.clone(), req)?;
                Ok(Some(Response::json(updated_game, 200, "OK")))
            },
            ("PUT", Some("guess"), None) => {
                let guess: GuessRequest = serde_json::from_str(std::str::from_utf8(request.msg.body.as_slice())?)?;
                let updated_game = self.service.guess(self.game_id.clone(), guess)?;
                Ok(Some(Response::json(updated_game, 200, "OK")))
            },
            ("PUT", Some("end-turn"), None) => {
                let updated_game = self.service.end_turn(self.game_id.clone())?;
                Ok(Some(Response::json(updated_game, 200, "OK")))
            },
            ("PUT", Some("guess"), Some("undo")) => {
                let updated_game = self.service.undo_guess(self.game_id.clone())?;
                Ok(Some(Response::json(updated_game, 200, "OK")))
            },
            _ => Ok(None),
        }
    }
}
