#[macro_use]
extern crate serde_json;

use std::fmt::Debug;

use codenames_domain::{
    game::{model::Player, service::GameService},
    GameNameBody, ServiceError, StdError,
};
use log::debug;
use serde_json::Value;
use urlencoding::decode;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

use crate::{
    dictionary::WordGeneratorWasmCloud,
    game::{board::BoardGeneratorWasmCloud, dao::WasmKeyValueDao},
};

mod dictionary;
mod game;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct CodenamesActor {}

impl CodenamesActor {
    pub fn path_segments<'l>(&self, req: &'l HttpRequest) -> Vec<&'l str> {
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
        let service = GameService::new(word_generator, board_generator, dao)
            .map_err(|e| std_to_rpc_error(e))?;

        debug!("Request received: Path is {}", req.path);

        let &method = &req.method.as_str();
        let segments = self.path_segments(req);

        let routing_result: Result<Value, ServiceError> = match (method, &segments[..]) {
            // get a random game key
            ("GET", [""]) => {
                debug_route("random game");
                let json = json!(service.random_name().await.map_err(|e| to_rpc_error(e))?);
                Ok(json)
            }

            // create a game
            ("POST", ["game"]) => {
                debug_route("create game");
                let body: GameNameBody = serde_json::from_str(
                    std::str::from_utf8(req.body.as_slice())
                        .map_err(|e| RpcError::Other(e.to_string()))?,
                )
                .map_err(|e| RpcError::Other(e.to_string()))?;
                let game = service
                    .new_game(body.game_name)
                    .await
                    .map_err(to_rpc_error)?;
                Ok(json!(game))
            }

            // get a list of all games
            ("GET", ["game"]) => {
                debug_route("get all games");
                let games = service.clone().find().await.map_err(to_rpc_error)?;
                Ok(json!(games))
            }

            // get an existing game
            ("GET", ["game", game_key]) => {
                debug_route("get game");
                let game = service
                    .clone()
                    .get(game_key, &None, &None)
                    .await
                    .map_err(to_rpc_error)?;
                Ok(json!(game))
            }

            // join a game as a player
            ("PUT", ["game", game_key, "join"]) => {
                debug_route("join");
                let player: Player = serde_json::from_str(
                    std::str::from_utf8(req.body.as_slice())
                        .map_err(|e| RpcError::Other(e.to_string()))?,
                )
                .map_err(|e| RpcError::Other(e.to_string()))?;
                let updated_game = service
                    .join(game_key.to_string(), player)
                    .await
                    .map_err(to_rpc_error)?;
                Ok(json!(updated_game))
            }

            // end the current team's turn
            ("PUT", ["game", game_key, "end-turn"]) => {
                debug_route("end turn");
                let updated_game = service
                    .end_turn(game_key.to_string())
                    .await
                    .map_err(to_rpc_error)?;
                Ok(json!(updated_game))
            }

            // get a player's view of the game
            ("GET", ["game", game_key, player_name_encoded]) => {
                debug_route("get player game");
                let player_name =
                    decode(player_name_encoded).map_err(|e| RpcError::Other(e.to_string()))?;
                let game = service
                    .clone()
                    .get(game_key, &Some(player_name), &None)
                    .await
                    .map_err(to_rpc_error)?;
                Ok(json!(game))
            }

            // guess a word
            ("PUT", ["game", game_key, player_name_encoded, "guess", index]) => {
                debug_route("guess");
                let player_name =
                    decode(player_name_encoded).map_err(|e| RpcError::Other(e.to_string()))?;
                let board_index_result = index.parse::<usize>();
                match board_index_result {
                    Ok(board_index) => {
                        let updated_game = service
                            .guess(game_key.to_string(), (player_name.as_str(), board_index))
                            .await
                            .map_err(to_rpc_error)?;
                        Ok(json!(updated_game))
                    }
                    Err(e) => Err(ServiceError::BadRequest(e.to_string())),
                }
            }

            // leave a game
            ("PUT", ["game", game_key, player_name_encoded, "leave"]) => {
                debug_route("leave");
                let player_name =
                    decode(player_name_encoded).map_err(|e| RpcError::Other(e.to_string()))?;
                let updated_game = service
                    .clone()
                    .leave(game_key.to_string(), player_name.as_str())
                    .await
                    .map_err(to_rpc_error)?;
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
                    },
                )
            })
    }
}

fn to_service_error(err: RpcError) -> ServiceError {
    ServiceError::Unknown(err.to_string())
}

fn std_to_rpc_error(err: StdError) -> RpcError {
    RpcError::Other(err.to_string())
}

fn to_rpc_error(err: ServiceError) -> RpcError {
    RpcError::Other(err.to_string())
}

fn debug_route(msg: &str) {
    debug!("matched route: {}", msg)
}
