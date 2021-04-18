use wasmcloud_actor_http_server::Response;

use codenames_domain::game::model::{GuessRequest, PlayerRequest};
use codenames_domain::game::service::Service;

use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::HandlerResult;

pub struct PlayerRouter {
    game_key: String,
    service: Service,
}

impl PlayerRouter {
    pub fn new(service: &Service, game_key: &String) -> Self {
        Self {
            game_key: game_key.clone(),
            service: service.clone(),
        }
    }
}

impl RoutedRequestHandler for PlayerRouter {
    fn handle(&self, request: &RoutedRequest) -> HandlerResult<Option<Response>> {
        match request.path_head.clone() {
            None => Ok(None),
            Some(player_name) => {
                match (
                    request.msg.method.as_str(),
                    request.path_tail.get(0).map(|s| s.as_str()),
                    request.path_tail.get(1).map(|s| s.as_str()),
                ) {
                    ("GET", None, None) => {
                        let game = self
                            .service
                            .clone()
                            .get(self.game_key.clone(), Some(PlayerRequest { player_name }))?;
                        Ok(Some(Response::json(game, 200, "OK")))
                    }
                    ("PUT", Some("guess"), Some(index)) => {
                        let board_index_result = index.parse::<usize>();
                        match board_index_result {
                            Ok(board_index) => {
                                let guess_request = GuessRequest {
                                    player_name,
                                    board_index,
                                };
                                let updated_game =
                                    self.service.guess(self.game_key.clone(), guess_request)?;
                                Ok(Some(Response::json(updated_game, 200, "OK")))
                            }
                            Err(_) => Ok(Some(Response::bad_request())),
                        }
                    }
                    ("PUT", Some("leave"), None) => {
                        let updated_game = self
                            .service
                            .clone()
                            .leave(self.game_key.clone(), PlayerRequest { player_name })?;
                        Ok(Some(Response::json(updated_game, 200, "OK")))
                    }
                    _ => Ok(None),
                }
            }
        }
    }
}
