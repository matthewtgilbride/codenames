#[macro_use]
extern crate serde_json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue};

use log::debug;
use serde_json::Value;
use urlencoding::decode;

use codenames_domain::game::model::{Player};
use codenames_domain::game::service::GameService;
use codenames_domain::{GameNameBody, ServiceError};

use crate::dictionary::WordGeneratorWasmCloud;
use crate::game::board::BoardGeneratorWasmCloud;
use crate::game::dao::WasmKeyValueDao;

mod dictionary;
mod game;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct CodenamesActor {}

impl CodenamesActor {
    pub fn path_segments(&self, req: &HttpRequest) -> Vec<&str> {
        req.path
            .trim_end_matches('/')
            .split('/')
            .skip(1)
            .collect::<Vec<_>>()
    }
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for CodenamesActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let word_generator = Box::new(WordGeneratorWasmCloud);
        let board_generator = Box::new(BoardGeneratorWasmCloud);
        let dao = Box::new(WasmKeyValueDao::new(ctx));
        let service = GameService::new(word_generator, board_generator, dao)?;

        debug!("Request received: Path is {}", req.path);

        let &method = &req.method.as_str();
        let segments = self.path_segments(req);

        let routing_result: Result<Value, ServiceError> = match (method, &segments[..]) {
            // get a random game key
            ("GET", [""]) => {
                debug_route("random game");
                let json = json!(service.random_name()?);
                Ok(json)
            }

            // create a game
            ("POST", ["game"]) => {
                debug_route("create game");
                let body: GameNameBody =
                    serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
                let game = service.new_game(body.game_name)?;
                Ok(json!(game))
            }

            // get a list of all games
            ("GET", ["game"]) => {
                debug_route("get all games");
                let games = service.clone().find()?;
                Ok(json!(games))
            }

            // get an existing game
            ("GET", ["game", game_key]) => {
                debug_route("get game");
                let game = service.clone().get(game_key, &None, &None)?;
                Ok(json!(game))
            }

            // join a game as a player
            ("PUT", ["game", game_key, "join"]) => {
                debug_route("join");
                let player: Player = serde_json::from_str(std::str::from_utf8(req.body.as_slice())?)?;
                let updated_game = service.join(game_key.to_string(), player)?;
                Ok(json!(updated_game))
            }

            // undo a guess
            ("PUT", ["game", game_key, "guess", "undo"]) => {
                debug_route("undo guess");
                let updated_game = service.undo_guess(game_key.to_string())?;
                Ok(json!(updated_game))
            }

            // end the current team's turn
            ("PUT", ["game", game_key, "end-turn"]) => {
                debug_route("end turn");
                let updated_game = service.end_turn(game_key.to_string())?;
                Ok(json!(updated_game))
            }

            // get a player's view of the game
            ("GET", ["game", game_key, player_name_encoded]) => {
                debug_route("get player game");
                let player_name = decode(player_name_encoded)?;
                let game = service.clone().get(
                    game_key,
                    &Some(player_name),
                    &None
                )?;
                Ok(json!(game))
            }

            // guess a word
            ("PUT", ["game", game_key, player_name_encoded, "guess", index]) => {
                debug_route("guess");
                let player_name = decode(player_name_encoded)?;
                let board_index_result = index.parse::<usize>();
                match board_index_result {
                    Ok(board_index) => {
                        let updated_game = service.guess(
                            game_key.to_string(),
                            (player_name.as_str(), board_index)
                        )?;
                        Ok(json!(updated_game))
                    }
                    Err(e) => Err(ServiceError::BadRequest(e.to_string())),
                }
            }

            // leave a game
            ("PUT", ["game", game_key, player_name_encoded, "leave"]) => {
                debug_route("leave");
                let player_name = decode(player_name_encoded)?;
                let updated_game = service.clone().leave(
                    game_key.to_string(),
                    player_name.as_str(),
                )?;
                Ok(json!(updated_game))
            }

            _ => Err(ServiceError::NotFound("unmatched route".to_string())),
        };

        routing_result
            .map(|json| HttpResponse::json(json, 200))
            .unwrap_or_else(|se| {
            HttpResponse::json(
                se.clone(),
                match se {
                    ServiceError::BadRequest(_) => 400,
                    ServiceError::NotFound(_) => 404,
                    ServiceError::Unknown(_) => 500,
                }
            )
        })
    }
}



fn debug_route(msg: &str) {
    debug!("matched route: {}", msg)
}
