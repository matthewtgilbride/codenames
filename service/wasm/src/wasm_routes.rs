extern crate wapc_guest as guest;

use actor_http_server as http;
use guest::prelude::*;

use codenames_domain::game::model::{Game, GuessRequest, LeaveRequest, NewGameRequest};
use codenames_domain::game::player::model::Player;
use codenames_domain::game::service::Service;
use codenames_domain::ServiceResult;

pub struct WasmRoutes {
    service: Service,
}

impl WasmRoutes {
    pub fn new(service: Service) -> WasmRoutes {
        WasmRoutes { service }
    }

    pub fn random_name(&self, _: http::Request) -> HandlerResult<http::Response> {
        let json = json!(self.service.random_name()?);
        Ok(http::Response::json(json, 200, "OK"))
    }

    pub fn new_game(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let body: NewGameRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let game = self.service.new_game(body)?;
        Ok(http::Response::json(game, 200, "OK"))
    }

    pub fn get(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let (_, game) = self.get_existing_game_by_key(msg)?;
        Ok(http::Response::json(game, 200, "OK"))
    }

    pub fn join(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let player: Player = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let (key, _) = self.get_existing_game_by_key(msg)?;
        let updated_game = self.service.join(key, player)?;
        Ok(http::Response::json(updated_game, 200, "OK"))
    }

    pub fn leave(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let req: LeaveRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let (key, _) = self.get_existing_game_by_key(msg)?;
        let updated_game = self.service.leave(key, req)?;
        Ok(http::Response::json(updated_game, 200, "OK"))
    }

    pub fn guess(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let guess: GuessRequest = serde_json::from_str(std::str::from_utf8(msg.body.as_slice())?)?;
        let (key, _) = self.get_existing_game_by_key(msg)?;
        let updated_game = self.service.guess(key, guess)?;
        Ok(http::Response::json(updated_game, 200, "OK"))
    }

    pub fn undo_guess(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let (key, _) = self.get_existing_game_by_key(msg)?;
        let updated_game = self.service.undo_guess(key)?;
        Ok(http::Response::json(updated_game, 200, "OK"))
    }

    pub fn end_turn(&mut self, msg: http::Request) -> HandlerResult<http::Response> {
        let (key, _) = self.get_existing_game_by_key(msg)?;
        let updated_game = self.service.end_turn(key)?;
        Ok(http::Response::json(updated_game, 200, "OK"))
    }

    fn get_existing_game_by_key(&mut self, msg: http::Request) -> ServiceResult<(String, Game)> {
        let game_key = get_game_key(msg.path);
        game_key.map_or_else(
            || Err("game key could not be found in path".into()),
            |key| self.service.get(key.clone()).map(|game| (key, game)),
        )
    }
}

fn get_game_key(path: String) -> Option<String> {
    path.split("/")
        .into_iter()
        .find(|&path_part| !is_path_segment(path_part.to_string()) && path_part.len() > 0)
        .map(|s| s.to_string())
}

fn is_path_segment(part: String) -> bool {
    ["game", "join", "leave", "guess", "undo", "end-turn"]
        .iter()
        .cloned()
        .find(|s| s == &part)
        .is_some()
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_path_segment_true() {
        assert!(super::is_path_segment("game".to_string()))
    }

    #[test]
    fn is_path_segment_false() {
        assert!(!super::is_path_segment("foo".to_string()))
    }

    #[test]
    fn get_game_key_valid_url() {
        let key = "aaaa-aaaa-aaaa-aaaa";
        let path = format!("/game/{}/join", key);
        let result = super::get_game_key(path);
        assert_eq!(key, result.unwrap())
    }

    #[test]
    fn get_game_key_invalid_url() {
        let path = "/game/join".to_string();
        let result = super::get_game_key(path);
        assert!(result.is_none())
    }
}
